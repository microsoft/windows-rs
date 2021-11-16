#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const DEFAULT_WEIGHT: u32 = 1u32;
pub const DISPID_SRGId: i32 = 1i32;
pub const DISPID_SRGRecoContext: i32 = 2i32;
pub const DISPID_SRGState: i32 = 3i32;
pub const DISPID_SRGRules: i32 = 4i32;
pub const DISPID_SRGReset: i32 = 5i32;
pub const DISPID_SRGCommit: i32 = 6i32;
pub const DISPID_SRGCmdLoadFromFile: i32 = 7i32;
pub const DISPID_SRGCmdLoadFromObject: i32 = 8i32;
pub const DISPID_SRGCmdLoadFromResource: i32 = 9i32;
pub const DISPID_SRGCmdLoadFromMemory: i32 = 10i32;
pub const DISPID_SRGCmdLoadFromProprietaryGrammar: i32 = 11i32;
pub const DISPID_SRGCmdSetRuleState: i32 = 12i32;
pub const DISPID_SRGCmdSetRuleIdState: i32 = 13i32;
pub const DISPID_SRGDictationLoad: i32 = 14i32;
pub const DISPID_SRGDictationUnload: i32 = 15i32;
pub const DISPID_SRGDictationSetState: i32 = 16i32;
pub const DISPID_SRGSetWordSequenceData: i32 = 17i32;
pub const DISPID_SRGSetTextSelection: i32 = 18i32;
pub const DISPID_SRGIsPronounceable: i32 = 19i32;
pub const DISPIDSPTSI_ActiveOffset: i32 = 1i32;
pub const DISPIDSPTSI_ActiveLength: i32 = 2i32;
pub const DISPIDSPTSI_SelectionOffset: i32 = 3i32;
pub const DISPIDSPTSI_SelectionLength: i32 = 4i32;
pub const DISPID_SAStatus: i32 = 200i32;
pub const DISPID_SABufferInfo: i32 = 201i32;
pub const DISPID_SADefaultFormat: i32 = 202i32;
pub const DISPID_SAVolume: i32 = 203i32;
pub const DISPID_SABufferNotifySize: i32 = 204i32;
pub const DISPID_SAEventHandle: i32 = 205i32;
pub const DISPID_SASetState: i32 = 206i32;
pub const DISPID_SABIMinNotification: i32 = 1i32;
pub const DISPID_SABIBufferSize: i32 = 2i32;
pub const DISPID_SABIEventBias: i32 = 3i32;
pub const DISPID_SAFType: i32 = 1i32;
pub const DISPID_SAFGuid: i32 = 2i32;
pub const DISPID_SAFGetWaveFormatEx: i32 = 3i32;
pub const DISPID_SAFSetWaveFormatEx: i32 = 4i32;
pub const DISPID_SASFreeBufferSpace: i32 = 1i32;
pub const DISPID_SASNonBlockingIO: i32 = 2i32;
pub const DISPID_SASState: i32 = 3i32;
pub const DISPID_SASCurrentSeekPosition: i32 = 4i32;
pub const DISPID_SASCurrentDevicePosition: i32 = 5i32;
pub const DISPID_SBSFormat: i32 = 1i32;
pub const DISPID_SBSRead: i32 = 2i32;
pub const DISPID_SBSWrite: i32 = 3i32;
pub const DISPID_SBSSeek: i32 = 4i32;
pub const DISPID_SCSBaseStream: i32 = 100i32;
pub const DISPID_SDKSetBinaryValue: i32 = 1i32;
pub const DISPID_SDKGetBinaryValue: i32 = 2i32;
pub const DISPID_SDKSetStringValue: i32 = 3i32;
pub const DISPID_SDKGetStringValue: i32 = 4i32;
pub const DISPID_SDKSetLongValue: i32 = 5i32;
pub const DISPID_SDKGetlongValue: i32 = 6i32;
pub const DISPID_SDKOpenKey: i32 = 7i32;
pub const DISPID_SDKCreateKey: i32 = 8i32;
pub const DISPID_SDKDeleteKey: i32 = 9i32;
pub const DISPID_SDKDeleteValue: i32 = 10i32;
pub const DISPID_SDKEnumKeys: i32 = 11i32;
pub const DISPID_SDKEnumValues: i32 = 12i32;
pub const DISPID_SFSOpen: i32 = 100i32;
pub const DISPID_SFSClose: i32 = 101i32;
pub const DISPID_SGRAttributes: i32 = 1i32;
pub const DISPID_SGRInitialState: i32 = 2i32;
pub const DISPID_SGRName: i32 = 3i32;
pub const DISPID_SGRId: i32 = 4i32;
pub const DISPID_SGRClear: i32 = 5i32;
pub const DISPID_SGRAddResource: i32 = 6i32;
pub const DISPID_SGRAddState: i32 = 7i32;
pub const DISPID_SGRSRule: i32 = 1i32;
pub const DISPID_SGRSTransitions: i32 = 2i32;
pub const DISPID_SGRSAddWordTransition: i32 = 3i32;
pub const DISPID_SGRSAddRuleTransition: i32 = 4i32;
pub const DISPID_SGRSAddSpecialTransition: i32 = 5i32;
pub const DISPID_SGRSTType: i32 = 1i32;
pub const DISPID_SGRSTText: i32 = 2i32;
pub const DISPID_SGRSTRule: i32 = 3i32;
pub const DISPID_SGRSTWeight: i32 = 4i32;
pub const DISPID_SGRSTPropertyName: i32 = 5i32;
pub const DISPID_SGRSTPropertyId: i32 = 6i32;
pub const DISPID_SGRSTPropertyValue: i32 = 7i32;
pub const DISPID_SGRSTNextState: i32 = 8i32;
pub const DISPID_SGRSTsCount: i32 = 1i32;
pub const DISPID_SGRSTsItem: i32 = 0i32;
pub const DISPID_SGRSTs_NewEnum: i32 = -4i32;
pub const DISPID_SGRsCount: i32 = 1i32;
pub const DISPID_SGRsDynamic: i32 = 2i32;
pub const DISPID_SGRsAdd: i32 = 3i32;
pub const DISPID_SGRsCommit: i32 = 4i32;
pub const DISPID_SGRsCommitAndSave: i32 = 5i32;
pub const DISPID_SGRsFindRule: i32 = 6i32;
pub const DISPID_SGRsItem: i32 = 0i32;
pub const DISPID_SGRs_NewEnum: i32 = -4i32;
pub const DISPID_SLGenerationId: i32 = 1i32;
pub const DISPID_SLGetWords: i32 = 2i32;
pub const DISPID_SLAddPronunciation: i32 = 3i32;
pub const DISPID_SLAddPronunciationByPhoneIds: i32 = 4i32;
pub const DISPID_SLRemovePronunciation: i32 = 5i32;
pub const DISPID_SLRemovePronunciationByPhoneIds: i32 = 6i32;
pub const DISPID_SLGetPronunciations: i32 = 7i32;
pub const DISPID_SLGetGenerationChange: i32 = 8i32;
pub const DISPID_SLPsCount: i32 = 1i32;
pub const DISPID_SLPsItem: i32 = 0i32;
pub const DISPID_SLPs_NewEnum: i32 = -4i32;
pub const DISPID_SLPType: i32 = 1i32;
pub const DISPID_SLPLangId: i32 = 2i32;
pub const DISPID_SLPPartOfSpeech: i32 = 3i32;
pub const DISPID_SLPPhoneIds: i32 = 4i32;
pub const DISPID_SLPSymbolic: i32 = 5i32;
pub const DISPID_SLWLangId: i32 = 1i32;
pub const DISPID_SLWType: i32 = 2i32;
pub const DISPID_SLWWord: i32 = 3i32;
pub const DISPID_SLWPronunciations: i32 = 4i32;
pub const DISPID_SLWsCount: i32 = 1i32;
pub const DISPID_SLWsItem: i32 = 0i32;
pub const DISPID_SLWs_NewEnum: i32 = -4i32;
pub const DISPID_SMSADeviceId: i32 = 300i32;
pub const DISPID_SMSALineId: i32 = 301i32;
pub const DISPID_SMSAMMHandle: i32 = 302i32;
pub const DISPID_SMSSetData: i32 = 100i32;
pub const DISPID_SMSGetData: i32 = 101i32;
pub const DISPID_SOTId: i32 = 1i32;
pub const DISPID_SOTDataKey: i32 = 2i32;
pub const DISPID_SOTCategory: i32 = 3i32;
pub const DISPID_SOTGetDescription: i32 = 4i32;
pub const DISPID_SOTSetId: i32 = 5i32;
pub const DISPID_SOTGetAttribute: i32 = 6i32;
pub const DISPID_SOTCreateInstance: i32 = 7i32;
pub const DISPID_SOTRemove: i32 = 8i32;
pub const DISPID_SOTGetStorageFileName: i32 = 9i32;
pub const DISPID_SOTRemoveStorageFileName: i32 = 10i32;
pub const DISPID_SOTIsUISupported: i32 = 11i32;
pub const DISPID_SOTDisplayUI: i32 = 12i32;
pub const DISPID_SOTMatchesAttributes: i32 = 13i32;
pub const DISPID_SOTCId: i32 = 1i32;
pub const DISPID_SOTCDefault: i32 = 2i32;
pub const DISPID_SOTCSetId: i32 = 3i32;
pub const DISPID_SOTCGetDataKey: i32 = 4i32;
pub const DISPID_SOTCEnumerateTokens: i32 = 5i32;
pub const DISPID_SOTsCount: i32 = 1i32;
pub const DISPID_SOTsItem: i32 = 0i32;
pub const DISPID_SOTs_NewEnum: i32 = -4i32;
pub const DISPID_SPCLangId: i32 = 1i32;
pub const DISPID_SPCPhoneToId: i32 = 2i32;
pub const DISPID_SPCIdToPhone: i32 = 3i32;
pub const DISPID_SPARecoResult: i32 = 1i32;
pub const DISPID_SPAStartElementInResult: i32 = 2i32;
pub const DISPID_SPANumberOfElementsInResult: i32 = 3i32;
pub const DISPID_SPAPhraseInfo: i32 = 4i32;
pub const DISPID_SPACommit: i32 = 5i32;
pub const DISPID_SPAsCount: i32 = 1i32;
pub const DISPID_SPAsItem: i32 = 0i32;
pub const DISPID_SPAs_NewEnum: i32 = -4i32;
pub const DISPID_SPPBRestorePhraseFromMemory: i32 = 1i32;
pub const DISPID_SPEAudioTimeOffset: i32 = 1i32;
pub const DISPID_SPEAudioSizeTime: i32 = 2i32;
pub const DISPID_SPEAudioStreamOffset: i32 = 3i32;
pub const DISPID_SPEAudioSizeBytes: i32 = 4i32;
pub const DISPID_SPERetainedStreamOffset: i32 = 5i32;
pub const DISPID_SPERetainedSizeBytes: i32 = 6i32;
pub const DISPID_SPEDisplayText: i32 = 7i32;
pub const DISPID_SPELexicalForm: i32 = 8i32;
pub const DISPID_SPEPronunciation: i32 = 9i32;
pub const DISPID_SPEDisplayAttributes: i32 = 10i32;
pub const DISPID_SPERequiredConfidence: i32 = 11i32;
pub const DISPID_SPEActualConfidence: i32 = 12i32;
pub const DISPID_SPEEngineConfidence: i32 = 13i32;
pub const DISPID_SPEsCount: i32 = 1i32;
pub const DISPID_SPEsItem: i32 = 0i32;
pub const DISPID_SPEs_NewEnum: i32 = -4i32;
pub const DISPID_SPILanguageId: i32 = 1i32;
pub const DISPID_SPIGrammarId: i32 = 2i32;
pub const DISPID_SPIStartTime: i32 = 3i32;
pub const DISPID_SPIAudioStreamPosition: i32 = 4i32;
pub const DISPID_SPIAudioSizeBytes: i32 = 5i32;
pub const DISPID_SPIRetainedSizeBytes: i32 = 6i32;
pub const DISPID_SPIAudioSizeTime: i32 = 7i32;
pub const DISPID_SPIRule: i32 = 8i32;
pub const DISPID_SPIProperties: i32 = 9i32;
pub const DISPID_SPIElements: i32 = 10i32;
pub const DISPID_SPIReplacements: i32 = 11i32;
pub const DISPID_SPIEngineId: i32 = 12i32;
pub const DISPID_SPIEnginePrivateData: i32 = 13i32;
pub const DISPID_SPISaveToMemory: i32 = 14i32;
pub const DISPID_SPIGetText: i32 = 15i32;
pub const DISPID_SPIGetDisplayAttributes: i32 = 16i32;
pub const DISPID_SPPsCount: i32 = 1i32;
pub const DISPID_SPPsItem: i32 = 0i32;
pub const DISPID_SPPs_NewEnum: i32 = -4i32;
pub const DISPID_SPPName: i32 = 1i32;
pub const DISPID_SPPId: i32 = 2i32;
pub const DISPID_SPPValue: i32 = 3i32;
pub const DISPID_SPPFirstElement: i32 = 4i32;
pub const DISPID_SPPNumberOfElements: i32 = 5i32;
pub const DISPID_SPPEngineConfidence: i32 = 6i32;
pub const DISPID_SPPConfidence: i32 = 7i32;
pub const DISPID_SPPParent: i32 = 8i32;
pub const DISPID_SPPChildren: i32 = 9i32;
pub const DISPID_SPRDisplayAttributes: i32 = 1i32;
pub const DISPID_SPRText: i32 = 2i32;
pub const DISPID_SPRFirstElement: i32 = 3i32;
pub const DISPID_SPRNumberOfElements: i32 = 4i32;
pub const DISPID_SPRsCount: i32 = 1i32;
pub const DISPID_SPRsItem: i32 = 0i32;
pub const DISPID_SPRs_NewEnum: i32 = -4i32;
pub const DISPID_SPRuleName: i32 = 1i32;
pub const DISPID_SPRuleId: i32 = 2i32;
pub const DISPID_SPRuleFirstElement: i32 = 3i32;
pub const DISPID_SPRuleNumberOfElements: i32 = 4i32;
pub const DISPID_SPRuleParent: i32 = 5i32;
pub const DISPID_SPRuleChildren: i32 = 6i32;
pub const DISPID_SPRuleConfidence: i32 = 7i32;
pub const DISPID_SPRuleEngineConfidence: i32 = 8i32;
pub const DISPID_SPRulesCount: i32 = 1i32;
pub const DISPID_SPRulesItem: i32 = 0i32;
pub const DISPID_SPRules_NewEnum: i32 = -4i32;
pub const DISPID_SRCRecognizer: i32 = 1i32;
pub const DISPID_SRCAudioInInterferenceStatus: i32 = 2i32;
pub const DISPID_SRCRequestedUIType: i32 = 3i32;
pub const DISPID_SRCVoice: i32 = 4i32;
pub const DISPID_SRAllowVoiceFormatMatchingOnNextSet: i32 = 5i32;
pub const DISPID_SRCVoicePurgeEvent: i32 = 6i32;
pub const DISPID_SRCEventInterests: i32 = 7i32;
pub const DISPID_SRCCmdMaxAlternates: i32 = 8i32;
pub const DISPID_SRCState: i32 = 9i32;
pub const DISPID_SRCRetainedAudio: i32 = 10i32;
pub const DISPID_SRCRetainedAudioFormat: i32 = 11i32;
pub const DISPID_SRCPause: i32 = 12i32;
pub const DISPID_SRCResume: i32 = 13i32;
pub const DISPID_SRCCreateGrammar: i32 = 14i32;
pub const DISPID_SRCCreateResultFromMemory: i32 = 15i32;
pub const DISPID_SRCBookmark: i32 = 16i32;
pub const DISPID_SRCSetAdaptationData: i32 = 17i32;
pub const DISPID_SRCEStartStream: i32 = 1i32;
pub const DISPID_SRCEEndStream: i32 = 2i32;
pub const DISPID_SRCEBookmark: i32 = 3i32;
pub const DISPID_SRCESoundStart: i32 = 4i32;
pub const DISPID_SRCESoundEnd: i32 = 5i32;
pub const DISPID_SRCEPhraseStart: i32 = 6i32;
pub const DISPID_SRCERecognition: i32 = 7i32;
pub const DISPID_SRCEHypothesis: i32 = 8i32;
pub const DISPID_SRCEPropertyNumberChange: i32 = 9i32;
pub const DISPID_SRCEPropertyStringChange: i32 = 10i32;
pub const DISPID_SRCEFalseRecognition: i32 = 11i32;
pub const DISPID_SRCEInterference: i32 = 12i32;
pub const DISPID_SRCERequestUI: i32 = 13i32;
pub const DISPID_SRCERecognizerStateChange: i32 = 14i32;
pub const DISPID_SRCEAdaptation: i32 = 15i32;
pub const DISPID_SRCERecognitionForOtherContext: i32 = 16i32;
pub const DISPID_SRCEAudioLevel: i32 = 17i32;
pub const DISPID_SRCEEnginePrivate: i32 = 18i32;
pub const DISPID_SRRRecoContext: i32 = 1i32;
pub const DISPID_SRRTimes: i32 = 2i32;
pub const DISPID_SRRAudioFormat: i32 = 3i32;
pub const DISPID_SRRPhraseInfo: i32 = 4i32;
pub const DISPID_SRRAlternates: i32 = 5i32;
pub const DISPID_SRRAudio: i32 = 6i32;
pub const DISPID_SRRSpeakAudio: i32 = 7i32;
pub const DISPID_SRRSaveToMemory: i32 = 8i32;
pub const DISPID_SRRDiscardResultInfo: i32 = 9i32;
pub const DISPID_SRRSetTextFeedback: i32 = 12i32;
pub const DISPID_SRRTStreamTime: i32 = 1i32;
pub const DISPID_SRRTLength: i32 = 2i32;
pub const DISPID_SRRTTickCount: i32 = 3i32;
pub const DISPID_SRRTOffsetFromStart: i32 = 4i32;
pub const DISPID_SRRecognizer: i32 = 1i32;
pub const DISPID_SRAllowAudioInputFormatChangesOnNextSet: i32 = 2i32;
pub const DISPID_SRAudioInput: i32 = 3i32;
pub const DISPID_SRAudioInputStream: i32 = 4i32;
pub const DISPID_SRIsShared: i32 = 5i32;
pub const DISPID_SRState: i32 = 6i32;
pub const DISPID_SRStatus: i32 = 7i32;
pub const DISPID_SRProfile: i32 = 8i32;
pub const DISPID_SREmulateRecognition: i32 = 9i32;
pub const DISPID_SRCreateRecoContext: i32 = 10i32;
pub const DISPID_SRGetFormat: i32 = 11i32;
pub const DISPID_SRSetPropertyNumber: i32 = 12i32;
pub const DISPID_SRGetPropertyNumber: i32 = 13i32;
pub const DISPID_SRSetPropertyString: i32 = 14i32;
pub const DISPID_SRGetPropertyString: i32 = 15i32;
pub const DISPID_SRIsUISupported: i32 = 16i32;
pub const DISPID_SRDisplayUI: i32 = 17i32;
pub const DISPID_SRGetRecognizers: i32 = 18i32;
pub const DISPID_SVGetAudioInputs: i32 = 19i32;
pub const DISPID_SVGetProfiles: i32 = 20i32;
pub const DISPID_SRSAudioStatus: i32 = 1i32;
pub const DISPID_SRSCurrentStreamPosition: i32 = 2i32;
pub const DISPID_SRSCurrentStreamNumber: i32 = 3i32;
pub const DISPID_SRSNumberOfActiveRules: i32 = 4i32;
pub const DISPID_SRSClsidEngine: i32 = 5i32;
pub const DISPID_SRSSupportedLanguages: i32 = 6i32;
pub const DISPID_SVStatus: i32 = 1i32;
pub const DISPID_SVVoice: i32 = 2i32;
pub const DISPID_SVAudioOutput: i32 = 3i32;
pub const DISPID_SVAudioOutputStream: i32 = 4i32;
pub const DISPID_SVRate: i32 = 5i32;
pub const DISPID_SVVolume: i32 = 6i32;
pub const DISPID_SVAllowAudioOuputFormatChangesOnNextSet: i32 = 7i32;
pub const DISPID_SVEventInterests: i32 = 8i32;
pub const DISPID_SVPriority: i32 = 9i32;
pub const DISPID_SVAlertBoundary: i32 = 10i32;
pub const DISPID_SVSyncronousSpeakTimeout: i32 = 11i32;
pub const DISPID_SVSpeak: i32 = 12i32;
pub const DISPID_SVSpeakStream: i32 = 13i32;
pub const DISPID_SVPause: i32 = 14i32;
pub const DISPID_SVResume: i32 = 15i32;
pub const DISPID_SVSkip: i32 = 16i32;
pub const DISPID_SVGetVoices: i32 = 17i32;
pub const DISPID_SVGetAudioOutputs: i32 = 18i32;
pub const DISPID_SVWaitUntilDone: i32 = 19i32;
pub const DISPID_SVSpeakCompleteEvent: i32 = 20i32;
pub const DISPID_SVIsUISupported: i32 = 21i32;
pub const DISPID_SVDisplayUI: i32 = 22i32;
pub const DISPID_SVEStreamStart: i32 = 1i32;
pub const DISPID_SVEStreamEnd: i32 = 2i32;
pub const DISPID_SVEVoiceChange: i32 = 3i32;
pub const DISPID_SVEBookmark: i32 = 4i32;
pub const DISPID_SVEWord: i32 = 5i32;
pub const DISPID_SVEPhoneme: i32 = 6i32;
pub const DISPID_SVESentenceBoundary: i32 = 7i32;
pub const DISPID_SVEViseme: i32 = 8i32;
pub const DISPID_SVEAudioLevel: i32 = 9i32;
pub const DISPID_SVEEnginePrivate: i32 = 10i32;
pub const DISPID_SVSCurrentStreamNumber: i32 = 1i32;
pub const DISPID_SVSLastStreamNumberQueued: i32 = 2i32;
pub const DISPID_SVSLastResult: i32 = 3i32;
pub const DISPID_SVSRunningState: i32 = 4i32;
pub const DISPID_SVSInputWordPosition: i32 = 5i32;
pub const DISPID_SVSInputWordLength: i32 = 6i32;
pub const DISPID_SVSInputSentencePosition: i32 = 7i32;
pub const DISPID_SVSInputSentenceLength: i32 = 8i32;
pub const DISPID_SVSLastBookmark: i32 = 9i32;
pub const DISPID_SVSLastBookmarkId: i32 = 10i32;
pub const DISPID_SVSPhonemeId: i32 = 11i32;
pub const DISPID_SVSVisemeId: i32 = 12i32;
pub const DISPID_SWFEFormatTag: i32 = 1i32;
pub const DISPID_SWFEChannels: i32 = 2i32;
pub const DISPID_SWFESamplesPerSec: i32 = 3i32;
pub const DISPID_SWFEAvgBytesPerSec: i32 = 4i32;
pub const DISPID_SWFEBlockAlign: i32 = 5i32;
pub const DISPID_SWFEBitsPerSample: i32 = 6i32;
pub const DISPID_SWFEExtraData: i32 = 7i32;
pub const DISPID_SRRGetXMLResult: i32 = 10i32;
pub const DISPID_SRRGetXMLErrorInfo: i32 = 11i32;
#[repr(transparent)]
pub struct IEnumSpObjectTokens(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumSpObjectTokens {}
impl ::core::clone::Clone for IEnumSpObjectTokens {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpAudio(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpAudio {}
impl ::core::clone::Clone for ISpAudio {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpContainerLexicon(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpContainerLexicon {}
impl ::core::clone::Clone for ISpContainerLexicon {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpDataKey(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpDataKey {}
impl ::core::clone::Clone for ISpDataKey {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpDisplayAlternates(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpDisplayAlternates {}
impl ::core::clone::Clone for ISpDisplayAlternates {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpEnginePronunciation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpEnginePronunciation {}
impl ::core::clone::Clone for ISpEnginePronunciation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpEventSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpEventSink {}
impl ::core::clone::Clone for ISpEventSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpEventSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpEventSource {}
impl ::core::clone::Clone for ISpEventSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpEventSource2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpEventSource2 {}
impl ::core::clone::Clone for ISpEventSource2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpGrammarBuilder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpGrammarBuilder {}
impl ::core::clone::Clone for ISpGrammarBuilder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpGrammarBuilder2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpGrammarBuilder2 {}
impl ::core::clone::Clone for ISpGrammarBuilder2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpLexicon(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpLexicon {}
impl ::core::clone::Clone for ISpLexicon {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpMMSysAudio(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpMMSysAudio {}
impl ::core::clone::Clone for ISpMMSysAudio {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpNotifyCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpNotifyCallback {}
impl ::core::clone::Clone for ISpNotifyCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpNotifySink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpNotifySink {}
impl ::core::clone::Clone for ISpNotifySink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpNotifySource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpNotifySource {}
impl ::core::clone::Clone for ISpNotifySource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpNotifyTranslator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpNotifyTranslator {}
impl ::core::clone::Clone for ISpNotifyTranslator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpObjectToken(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpObjectToken {}
impl ::core::clone::Clone for ISpObjectToken {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpObjectTokenCategory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpObjectTokenCategory {}
impl ::core::clone::Clone for ISpObjectTokenCategory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpObjectTokenInit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpObjectTokenInit {}
impl ::core::clone::Clone for ISpObjectTokenInit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpObjectWithToken(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpObjectWithToken {}
impl ::core::clone::Clone for ISpObjectWithToken {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpPhoneConverter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpPhoneConverter {}
impl ::core::clone::Clone for ISpPhoneConverter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpPhoneticAlphabetConverter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpPhoneticAlphabetConverter {}
impl ::core::clone::Clone for ISpPhoneticAlphabetConverter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpPhoneticAlphabetSelection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpPhoneticAlphabetSelection {}
impl ::core::clone::Clone for ISpPhoneticAlphabetSelection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpPhrase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpPhrase {}
impl ::core::clone::Clone for ISpPhrase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpPhrase2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpPhrase2 {}
impl ::core::clone::Clone for ISpPhrase2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpPhraseAlt(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpPhraseAlt {}
impl ::core::clone::Clone for ISpPhraseAlt {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpProperties {}
impl ::core::clone::Clone for ISpProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpRecoContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpRecoContext {}
impl ::core::clone::Clone for ISpRecoContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpRecoContext2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpRecoContext2 {}
impl ::core::clone::Clone for ISpRecoContext2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpRecoGrammar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpRecoGrammar {}
impl ::core::clone::Clone for ISpRecoGrammar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpRecoGrammar2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpRecoGrammar2 {}
impl ::core::clone::Clone for ISpRecoGrammar2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpRecoResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpRecoResult {}
impl ::core::clone::Clone for ISpRecoResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpRecoResult2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpRecoResult2 {}
impl ::core::clone::Clone for ISpRecoResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpRecognizer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpRecognizer {}
impl ::core::clone::Clone for ISpRecognizer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpRecognizer2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpRecognizer2 {}
impl ::core::clone::Clone for ISpRecognizer2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpRegDataKey(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpRegDataKey {}
impl ::core::clone::Clone for ISpRegDataKey {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpResourceManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpResourceManager {}
impl ::core::clone::Clone for ISpResourceManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpSerializeState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpSerializeState {}
impl ::core::clone::Clone for ISpSerializeState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpShortcut(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpShortcut {}
impl ::core::clone::Clone for ISpShortcut {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpStream {}
impl ::core::clone::Clone for ISpStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpStreamFormat(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpStreamFormat {}
impl ::core::clone::Clone for ISpStreamFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpStreamFormatConverter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpStreamFormatConverter {}
impl ::core::clone::Clone for ISpStreamFormatConverter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpTranscript(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpTranscript {}
impl ::core::clone::Clone for ISpTranscript {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpVoice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpVoice {}
impl ::core::clone::Clone for ISpVoice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpXMLRecoResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpXMLRecoResult {}
impl ::core::clone::Clone for ISpXMLRecoResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechAudio(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechAudio {}
impl ::core::clone::Clone for ISpeechAudio {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechAudioBufferInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechAudioBufferInfo {}
impl ::core::clone::Clone for ISpeechAudioBufferInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechAudioFormat(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechAudioFormat {}
impl ::core::clone::Clone for ISpeechAudioFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechAudioStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechAudioStatus {}
impl ::core::clone::Clone for ISpeechAudioStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechBaseStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechBaseStream {}
impl ::core::clone::Clone for ISpeechBaseStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechCustomStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechCustomStream {}
impl ::core::clone::Clone for ISpeechCustomStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechDataKey(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechDataKey {}
impl ::core::clone::Clone for ISpeechDataKey {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechFileStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechFileStream {}
impl ::core::clone::Clone for ISpeechFileStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechGrammarRule(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechGrammarRule {}
impl ::core::clone::Clone for ISpeechGrammarRule {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechGrammarRuleState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechGrammarRuleState {}
impl ::core::clone::Clone for ISpeechGrammarRuleState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechGrammarRuleStateTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechGrammarRuleStateTransition {}
impl ::core::clone::Clone for ISpeechGrammarRuleStateTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechGrammarRuleStateTransitions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechGrammarRuleStateTransitions {}
impl ::core::clone::Clone for ISpeechGrammarRuleStateTransitions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechGrammarRules(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechGrammarRules {}
impl ::core::clone::Clone for ISpeechGrammarRules {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechLexicon(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechLexicon {}
impl ::core::clone::Clone for ISpeechLexicon {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechLexiconPronunciation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechLexiconPronunciation {}
impl ::core::clone::Clone for ISpeechLexiconPronunciation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechLexiconPronunciations(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechLexiconPronunciations {}
impl ::core::clone::Clone for ISpeechLexiconPronunciations {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechLexiconWord(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechLexiconWord {}
impl ::core::clone::Clone for ISpeechLexiconWord {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechLexiconWords(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechLexiconWords {}
impl ::core::clone::Clone for ISpeechLexiconWords {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechMMSysAudio(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechMMSysAudio {}
impl ::core::clone::Clone for ISpeechMMSysAudio {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechMemoryStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechMemoryStream {}
impl ::core::clone::Clone for ISpeechMemoryStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechObjectToken(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechObjectToken {}
impl ::core::clone::Clone for ISpeechObjectToken {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechObjectTokenCategory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechObjectTokenCategory {}
impl ::core::clone::Clone for ISpeechObjectTokenCategory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechObjectTokens(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechObjectTokens {}
impl ::core::clone::Clone for ISpeechObjectTokens {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechPhoneConverter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechPhoneConverter {}
impl ::core::clone::Clone for ISpeechPhoneConverter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechPhraseAlternate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechPhraseAlternate {}
impl ::core::clone::Clone for ISpeechPhraseAlternate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechPhraseAlternates(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechPhraseAlternates {}
impl ::core::clone::Clone for ISpeechPhraseAlternates {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechPhraseElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechPhraseElement {}
impl ::core::clone::Clone for ISpeechPhraseElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechPhraseElements(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechPhraseElements {}
impl ::core::clone::Clone for ISpeechPhraseElements {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechPhraseInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechPhraseInfo {}
impl ::core::clone::Clone for ISpeechPhraseInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechPhraseInfoBuilder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechPhraseInfoBuilder {}
impl ::core::clone::Clone for ISpeechPhraseInfoBuilder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechPhraseProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechPhraseProperties {}
impl ::core::clone::Clone for ISpeechPhraseProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechPhraseProperty(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechPhraseProperty {}
impl ::core::clone::Clone for ISpeechPhraseProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechPhraseReplacement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechPhraseReplacement {}
impl ::core::clone::Clone for ISpeechPhraseReplacement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechPhraseReplacements(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechPhraseReplacements {}
impl ::core::clone::Clone for ISpeechPhraseReplacements {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechPhraseRule(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechPhraseRule {}
impl ::core::clone::Clone for ISpeechPhraseRule {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechPhraseRules(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechPhraseRules {}
impl ::core::clone::Clone for ISpeechPhraseRules {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecoContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecoContext {}
impl ::core::clone::Clone for ISpeechRecoContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecoGrammar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecoGrammar {}
impl ::core::clone::Clone for ISpeechRecoGrammar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecoResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecoResult {}
impl ::core::clone::Clone for ISpeechRecoResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecoResult2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecoResult2 {}
impl ::core::clone::Clone for ISpeechRecoResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecoResultDispatch(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecoResultDispatch {}
impl ::core::clone::Clone for ISpeechRecoResultDispatch {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecoResultTimes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecoResultTimes {}
impl ::core::clone::Clone for ISpeechRecoResultTimes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognizer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognizer {}
impl ::core::clone::Clone for ISpeechRecognizer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognizerStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognizerStatus {}
impl ::core::clone::Clone for ISpeechRecognizerStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechResourceLoader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechResourceLoader {}
impl ::core::clone::Clone for ISpeechResourceLoader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechTextSelectionInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechTextSelectionInformation {}
impl ::core::clone::Clone for ISpeechTextSelectionInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechVoice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechVoice {}
impl ::core::clone::Clone for ISpeechVoice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechVoiceStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechVoiceStatus {}
impl ::core::clone::Clone for ISpeechVoiceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechWaveFormatEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechWaveFormatEx {}
impl ::core::clone::Clone for ISpeechWaveFormatEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechXMLRecoResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechXMLRecoResult {}
impl ::core::clone::Clone for ISpeechXMLRecoResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PA_Ipa: i32 = 0i32;
pub const PA_Ups: i32 = 1i32;
pub const PA_Sapi: i32 = 2i32;
pub const SAPI_ERROR_BASE: u32 = 20480u32;
pub const SPAR_Unknown: i32 = 0i32;
pub const SPAR_Low: i32 = 1i32;
pub const SPAR_Medium: i32 = 2i32;
pub const SPAR_High: i32 = 3i32;
pub const SPADS_Default: i32 = 0i32;
pub const SPADS_CurrentRecognizer: i32 = 1i32;
pub const SPADS_RecoProfile: i32 = 2i32;
pub const SPADS_Immediate: i32 = 4i32;
pub const SPADS_Reset: i32 = 8i32;
pub const SPADS_HighVolumeDataSource: i32 = 16i32;
#[repr(C)]
pub struct SPAUDIOBUFFERINFO {
    pub ulMsMinNotification: u32,
    pub ulMsBufferSize: u32,
    pub ulMsEventBias: u32,
}
impl ::core::marker::Copy for SPAUDIOBUFFERINFO {}
impl ::core::clone::Clone for SPAUDIOBUFFERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPAO_NONE: i32 = 0i32;
pub const SPAO_RETAIN_AUDIO: i32 = 1i32;
pub const SPAS_CLOSED: i32 = 0i32;
pub const SPAS_STOP: i32 = 1i32;
pub const SPAS_PAUSE: i32 = 2i32;
pub const SPAS_RUN: i32 = 3i32;
#[repr(C)]
pub struct SPAUDIOSTATUS {
    pub cbFreeBuffSpace: i32,
    pub cbNonBlockingIO: u32,
    pub State: SPAUDIOSTATE,
    pub CurSeekPos: u64,
    pub CurDevicePos: u64,
    pub dwAudioLevel: u32,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for SPAUDIOSTATUS {}
impl ::core::clone::Clone for SPAUDIOSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SPBINARYGRAMMAR {
    pub ulTotalSerializedSize: u32,
}
impl ::core::marker::Copy for SPBINARYGRAMMAR {}
impl ::core::clone::Clone for SPBINARYGRAMMAR {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPBO_NONE: i32 = 0i32;
pub const SPBO_PAUSE: i32 = 1i32;
pub const SPBO_AHEAD: i32 = 2i32;
pub const SPBO_TIME_UNITS: i32 = 4i32;
pub const SPRAF_TopLevel: i32 = 1i32;
pub const SPRAF_Active: i32 = 2i32;
pub const SPRAF_Export: i32 = 4i32;
pub const SPRAF_Import: i32 = 8i32;
pub const SPRAF_Interpreter: i32 = 16i32;
pub const SPRAF_Dynamic: i32 = 32i32;
pub const SPRAF_Root: i32 = 64i32;
pub const SPRAF_AutoPause: i32 = 65536i32;
pub const SPRAF_UserDelimited: i32 = 131072i32;
pub const SPCF_NONE: i32 = 0i32;
pub const SPCF_ADD_TO_USER_LEXICON: i32 = 1i32;
pub const SPCF_DEFINITE_CORRECTION: i32 = 2i32;
pub const SPCS_DISABLED: i32 = 0i32;
pub const SPCS_ENABLED: i32 = 1i32;
pub const SPDKL_DefaultLocation: i32 = 0i32;
pub const SPDKL_CurrentUser: i32 = 1i32;
pub const SPDKL_LocalMachine: i32 = 2i32;
pub const SPDKL_CurrentConfig: i32 = 5i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SPDISPLAYPHRASE {
    pub ulNumTokens: u32,
    pub pTokens: *mut SPDISPLAYTOKEN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPDISPLAYPHRASE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPDISPLAYPHRASE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SPDISPLAYTOKEN {
    pub pszLexical: super::super::Foundation::PWSTR,
    pub pszDisplay: super::super::Foundation::PWSTR,
    pub bDisplayAttributes: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPDISPLAYTOKEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPDISPLAYTOKEN {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPAF_ONE_TRAILING_SPACE: i32 = 2i32;
pub const SPAF_TWO_TRAILING_SPACES: i32 = 4i32;
pub const SPAF_CONSUME_LEADING_SPACES: i32 = 8i32;
pub const SPAF_BUFFER_POSITION: i32 = 16i32;
pub const SPAF_ALL: i32 = 31i32;
pub const SPAF_USER_SPECIFIED: i32 = 128i32;
pub const SPF_DEFAULT: i32 = 0i32;
pub const SPF_ASYNC: i32 = 1i32;
pub const SPF_PURGEBEFORESPEAK: i32 = 2i32;
pub const SPF_IS_FILENAME: i32 = 4i32;
pub const SPF_IS_XML: i32 = 8i32;
pub const SPF_IS_NOT_XML: i32 = 16i32;
pub const SPF_PERSIST_XML: i32 = 32i32;
pub const SPF_NLP_SPEAK_PUNC: i32 = 64i32;
pub const SPF_PARSE_SAPI: i32 = 128i32;
pub const SPF_PARSE_SSML: i32 = 256i32;
pub const SPF_PARSE_AUTODETECT: i32 = 0i32;
pub const SPF_NLP_MASK: i32 = 64i32;
pub const SPF_PARSE_MASK: i32 = 384i32;
pub const SPF_VOICE_MASK: i32 = 511i32;
pub const SPF_UNUSED_FLAGS: i32 = -512i32;
pub const SPESF_NONE: i32 = 0i32;
pub const SPESF_STREAM_RELEASED: i32 = 1i32;
pub const SPESF_EMULATED: i32 = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SPEVENT {
    pub _bitfield: i32,
    pub ulStreamNum: u32,
    pub ullAudioStreamOffset: u64,
    pub wParam: super::super::Foundation::WPARAM,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPEVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPEI_UNDEFINED: i32 = 0i32;
pub const SPEI_START_INPUT_STREAM: i32 = 1i32;
pub const SPEI_END_INPUT_STREAM: i32 = 2i32;
pub const SPEI_VOICE_CHANGE: i32 = 3i32;
pub const SPEI_TTS_BOOKMARK: i32 = 4i32;
pub const SPEI_WORD_BOUNDARY: i32 = 5i32;
pub const SPEI_PHONEME: i32 = 6i32;
pub const SPEI_SENTENCE_BOUNDARY: i32 = 7i32;
pub const SPEI_VISEME: i32 = 8i32;
pub const SPEI_TTS_AUDIO_LEVEL: i32 = 9i32;
pub const SPEI_TTS_PRIVATE: i32 = 15i32;
pub const SPEI_MIN_TTS: i32 = 1i32;
pub const SPEI_MAX_TTS: i32 = 15i32;
pub const SPEI_END_SR_STREAM: i32 = 34i32;
pub const SPEI_SOUND_START: i32 = 35i32;
pub const SPEI_SOUND_END: i32 = 36i32;
pub const SPEI_PHRASE_START: i32 = 37i32;
pub const SPEI_RECOGNITION: i32 = 38i32;
pub const SPEI_HYPOTHESIS: i32 = 39i32;
pub const SPEI_SR_BOOKMARK: i32 = 40i32;
pub const SPEI_PROPERTY_NUM_CHANGE: i32 = 41i32;
pub const SPEI_PROPERTY_STRING_CHANGE: i32 = 42i32;
pub const SPEI_FALSE_RECOGNITION: i32 = 43i32;
pub const SPEI_INTERFERENCE: i32 = 44i32;
pub const SPEI_REQUEST_UI: i32 = 45i32;
pub const SPEI_RECO_STATE_CHANGE: i32 = 46i32;
pub const SPEI_ADAPTATION: i32 = 47i32;
pub const SPEI_START_SR_STREAM: i32 = 48i32;
pub const SPEI_RECO_OTHER_CONTEXT: i32 = 49i32;
pub const SPEI_SR_AUDIO_LEVEL: i32 = 50i32;
pub const SPEI_SR_RETAINEDAUDIO: i32 = 51i32;
pub const SPEI_SR_PRIVATE: i32 = 52i32;
pub const SPEI_RESERVED4: i32 = 53i32;
pub const SPEI_RESERVED5: i32 = 54i32;
pub const SPEI_RESERVED6: i32 = 55i32;
pub const SPEI_MIN_SR: i32 = 34i32;
pub const SPEI_MAX_SR: i32 = 55i32;
pub const SPEI_RESERVED1: i32 = 30i32;
pub const SPEI_RESERVED2: i32 = 33i32;
pub const SPEI_RESERVED3: i32 = 63i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SPEVENTEX {
    pub _bitfield: i32,
    pub ulStreamNum: u32,
    pub ullAudioStreamOffset: u64,
    pub wParam: super::super::Foundation::WPARAM,
    pub lParam: super::super::Foundation::LPARAM,
    pub ullAudioTimeOffset: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPEVENTEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPEVENTEX {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPET_LPARAM_IS_UNDEFINED: i32 = 0i32;
pub const SPET_LPARAM_IS_TOKEN: i32 = 1i32;
pub const SPET_LPARAM_IS_OBJECT: i32 = 2i32;
pub const SPET_LPARAM_IS_POINTER: i32 = 3i32;
pub const SPET_LPARAM_IS_STRING: i32 = 4i32;
#[repr(C)]
pub struct SPEVENTSOURCEINFO {
    pub ullEventInterest: u64,
    pub ullQueuedInterest: u64,
    pub ulCount: u32,
}
impl ::core::marker::Copy for SPEVENTSOURCEINFO {}
impl ::core::clone::Clone for SPEVENTSOURCEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPFM_OPEN_READONLY: i32 = 0i32;
pub const SPFM_OPEN_READWRITE: i32 = 1i32;
pub const SPFM_CREATE: i32 = 2i32;
pub const SPFM_CREATE_ALWAYS: i32 = 3i32;
pub const SPFM_NUM_MODES: i32 = 4i32;
pub const SPGO_SAPI: i32 = 1i32;
pub const SPGO_SRGS: i32 = 2i32;
pub const SPGO_UPS: i32 = 4i32;
pub const SPGO_SRGS_MS_SCRIPT: i32 = 8i32;
pub const SPGO_SRGS_W3C_SCRIPT: i32 = 256i32;
pub const SPGO_SRGS_STG_SCRIPT: i32 = 512i32;
pub const SPGO_SRGS_SCRIPT: i32 = 778i32;
pub const SPGO_FILE: i32 = 16i32;
pub const SPGO_HTTP: i32 = 32i32;
pub const SPGO_RES: i32 = 64i32;
pub const SPGO_OBJECT: i32 = 128i32;
pub const SPGO_DEFAULT: i32 = 1019i32;
pub const SPGO_ALL: i32 = 1023i32;
pub const SPGS_DISABLED: i32 = 0i32;
pub const SPGS_ENABLED: i32 = 1i32;
pub const SPGS_EXCLUSIVE: i32 = 3i32;
pub const SPWT_DISPLAY: i32 = 0i32;
pub const SPWT_LEXICAL: i32 = 1i32;
pub const SPWT_PRONUNCIATION: i32 = 2i32;
pub const SPWT_LEXICAL_NO_SPECIAL_CHARS: i32 = 3i32;
pub const SPINTERFERENCE_NONE: i32 = 0i32;
pub const SPINTERFERENCE_NOISE: i32 = 1i32;
pub const SPINTERFERENCE_NOSIGNAL: i32 = 2i32;
pub const SPINTERFERENCE_TOOLOUD: i32 = 3i32;
pub const SPINTERFERENCE_TOOQUIET: i32 = 4i32;
pub const SPINTERFERENCE_TOOFAST: i32 = 5i32;
pub const SPINTERFERENCE_TOOSLOW: i32 = 6i32;
pub const SPINTERFERENCE_LATENCY_WARNING: i32 = 7i32;
pub const SPINTERFERENCE_LATENCY_TRUNCATE_BEGIN: i32 = 8i32;
pub const SPINTERFERENCE_LATENCY_TRUNCATE_END: i32 = 9i32;
pub const eLEXTYPE_USER: i32 = 1i32;
pub const eLEXTYPE_APP: i32 = 2i32;
pub const eLEXTYPE_VENDORLEXICON: i32 = 4i32;
pub const eLEXTYPE_LETTERTOSOUND: i32 = 8i32;
pub const eLEXTYPE_MORPHOLOGY: i32 = 16i32;
pub const eLEXTYPE_RESERVED4: i32 = 32i32;
pub const eLEXTYPE_USER_SHORTCUT: i32 = 64i32;
pub const eLEXTYPE_RESERVED6: i32 = 128i32;
pub const eLEXTYPE_RESERVED7: i32 = 256i32;
pub const eLEXTYPE_RESERVED8: i32 = 512i32;
pub const eLEXTYPE_RESERVED9: i32 = 1024i32;
pub const eLEXTYPE_RESERVED10: i32 = 2048i32;
pub const eLEXTYPE_PRIVATE1: i32 = 4096i32;
pub const eLEXTYPE_PRIVATE2: i32 = 8192i32;
pub const eLEXTYPE_PRIVATE3: i32 = 16384i32;
pub const eLEXTYPE_PRIVATE4: i32 = 32768i32;
pub const eLEXTYPE_PRIVATE5: i32 = 65536i32;
pub const eLEXTYPE_PRIVATE6: i32 = 131072i32;
pub const eLEXTYPE_PRIVATE7: i32 = 262144i32;
pub const eLEXTYPE_PRIVATE8: i32 = 524288i32;
pub const eLEXTYPE_PRIVATE9: i32 = 1048576i32;
pub const eLEXTYPE_PRIVATE10: i32 = 2097152i32;
pub const eLEXTYPE_PRIVATE11: i32 = 4194304i32;
pub const eLEXTYPE_PRIVATE12: i32 = 8388608i32;
pub const eLEXTYPE_PRIVATE13: i32 = 16777216i32;
pub const eLEXTYPE_PRIVATE14: i32 = 33554432i32;
pub const eLEXTYPE_PRIVATE15: i32 = 67108864i32;
pub const eLEXTYPE_PRIVATE16: i32 = 134217728i32;
pub const eLEXTYPE_PRIVATE17: i32 = 268435456i32;
pub const eLEXTYPE_PRIVATE18: i32 = 536870912i32;
pub const eLEXTYPE_PRIVATE19: i32 = 1073741824i32;
pub const eLEXTYPE_PRIVATE20: i32 = -2147483648i32;
pub const SPLO_STATIC: i32 = 0i32;
pub const SPLO_DYNAMIC: i32 = 1i32;
pub const AllWords: i32 = 0i32;
pub const Subsequence: i32 = 1i32;
pub const OrderedSubset: i32 = 3i32;
pub const SubsequenceContentRequired: i32 = 5i32;
pub const OrderedSubsetContentRequired: i32 = 7i32;
#[repr(C)]
pub struct SPNORMALIZATIONLIST {
    pub ulSize: u32,
    pub ppszzNormalizedList: *mut *mut u16,
}
impl ::core::marker::Copy for SPNORMALIZATIONLIST {}
impl ::core::clone::Clone for SPNORMALIZATIONLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type SPNOTIFYCALLBACK = unsafe extern "system" fn(wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM);
pub const SPPS_NotOverriden: i32 = -1i32;
pub const SPPS_Unknown: i32 = 0i32;
pub const SPPS_Noun: i32 = 4096i32;
pub const SPPS_Verb: i32 = 8192i32;
pub const SPPS_Modifier: i32 = 12288i32;
pub const SPPS_Function: i32 = 16384i32;
pub const SPPS_Interjection: i32 = 20480i32;
pub const SPPS_Noncontent: i32 = 24576i32;
pub const SPPS_LMA: i32 = 28672i32;
pub const SPPS_SuppressWord: i32 = 61440i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct SPPHRASE {
    pub __AnonymousBase_sapi53_L5821_C34: SPPHRASE_50,
    pub pSML: super::super::Foundation::PWSTR,
    pub pSemanticErrorInfo: *mut SPSEMANTICERRORINFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for SPPHRASE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for SPPHRASE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SPPHRASEELEMENT {
    pub ulAudioTimeOffset: u32,
    pub ulAudioSizeTime: u32,
    pub ulAudioStreamOffset: u32,
    pub ulAudioSizeBytes: u32,
    pub ulRetainedStreamOffset: u32,
    pub ulRetainedSizeBytes: u32,
    pub pszDisplayText: super::super::Foundation::PWSTR,
    pub pszLexicalForm: super::super::Foundation::PWSTR,
    pub pszPronunciation: *mut u16,
    pub bDisplayAttributes: u8,
    pub RequiredConfidence: i8,
    pub ActualConfidence: i8,
    pub Reserved: u8,
    pub SREngineConfidence: f32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPPHRASEELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPPHRASEELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct SPPHRASEPROPERTY {
    pub pszName: super::super::Foundation::PWSTR,
    pub Anonymous: SPPHRASEPROPERTY_0,
    pub pszValue: super::super::Foundation::PWSTR,
    pub vValue: super::super::System::Com::VARIANT,
    pub ulFirstElement: u32,
    pub ulCountOfElements: u32,
    pub pNextSibling: *mut SPPHRASEPROPERTY,
    pub pFirstChild: *mut SPPHRASEPROPERTY,
    pub SREngineConfidence: f32,
    pub Confidence: i8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for SPPHRASEPROPERTY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for SPPHRASEPROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub union SPPHRASEPROPERTY_0 {
    pub ulId: u32,
    pub Anonymous: SPPHRASEPROPERTY_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for SPPHRASEPROPERTY_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for SPPHRASEPROPERTY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct SPPHRASEPROPERTY_0_0 {
    pub bType: u8,
    pub bReserved: u8,
    pub usArrayIndex: u16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for SPPHRASEPROPERTY_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for SPPHRASEPROPERTY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPPPUT_UNUSED: i32 = 0i32;
pub const SPPPUT_ARRAY_INDEX: i32 = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SPPHRASEREPLACEMENT {
    pub bDisplayAttributes: u8,
    pub pszReplacementText: super::super::Foundation::PWSTR,
    pub ulFirstElement: u32,
    pub ulCountOfElements: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPPHRASEREPLACEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPPHRASEREPLACEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPPR_ALL_ELEMENTS: i32 = -1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SPPHRASERULE {
    pub pszName: super::super::Foundation::PWSTR,
    pub ulId: u32,
    pub ulFirstElement: u32,
    pub ulCountOfElements: u32,
    pub pNextSibling: *mut SPPHRASERULE,
    pub pFirstChild: *mut SPPHRASERULE,
    pub SREngineConfidence: f32,
    pub Confidence: i8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPPHRASERULE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPPHRASERULE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct SPPHRASE_50 {
    pub cbSize: u32,
    pub LangID: u16,
    pub wHomophoneGroupId: u16,
    pub ullGrammarID: u64,
    pub ftStartTime: u64,
    pub ullAudioStreamPosition: u64,
    pub ulAudioSizeBytes: u32,
    pub ulRetainedSizeBytes: u32,
    pub ulAudioSizeTime: u32,
    pub Rule: SPPHRASERULE,
    pub pProperties: *mut SPPHRASEPROPERTY,
    pub pElements: *mut SPPHRASEELEMENT,
    pub cReplacements: u32,
    pub pReplacements: *mut SPPHRASEREPLACEMENT,
    pub SREngineID: ::windows_sys::core::GUID,
    pub ulSREnginePrivateDataSize: u32,
    pub pSREnginePrivateData: *mut u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for SPPHRASE_50 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for SPPHRASE_50 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ePRONFLAG_USED: i32 = 1i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct SPPROPERTYINFO {
    pub pszName: super::super::Foundation::PWSTR,
    pub ulId: u32,
    pub pszValue: super::super::Foundation::PWSTR,
    pub vValue: super::super::System::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for SPPROPERTYINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for SPPROPERTYINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SPRECOCONTEXTSTATUS {
    pub eInterference: SPINTERFERENCE,
    pub szRequestTypeOfUI: [u16; 255],
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for SPRECOCONTEXTSTATUS {}
impl ::core::clone::Clone for SPRECOCONTEXTSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPREF_AutoPause: i32 = 1i32;
pub const SPREF_Emulated: i32 = 2i32;
pub const SPREF_SMLTimeout: i32 = 4i32;
pub const SPREF_ExtendableParse: i32 = 8i32;
pub const SPREF_ReSent: i32 = 16i32;
pub const SPREF_Hypothesis: i32 = 32i32;
pub const SPREF_FalseRecognition: i32 = 64i32;
#[repr(C)]
pub struct SPRECOGNIZERSTATUS {
    pub AudioStatus: SPAUDIOSTATUS,
    pub ullRecognitionStreamPos: u64,
    pub ulStreamNumber: u32,
    pub ulNumActive: u32,
    pub clsidEngine: ::windows_sys::core::GUID,
    pub cLangIDs: u32,
    pub aLangID: [u16; 20],
    pub ullRecognitionStreamTime: u64,
}
impl ::core::marker::Copy for SPRECOGNIZERSTATUS {}
impl ::core::clone::Clone for SPRECOGNIZERSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SPRECORESULTTIMES {
    pub ftStreamTime: super::super::Foundation::FILETIME,
    pub ullLength: u64,
    pub dwTickCount: u32,
    pub ullStart: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPRECORESULTTIMES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPRECORESULTTIMES {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPRST_INACTIVE: i32 = 0i32;
pub const SPRST_ACTIVE: i32 = 1i32;
pub const SPRST_ACTIVE_ALWAYS: i32 = 2i32;
pub const SPRST_INACTIVE_WITH_PURGE: i32 = 3i32;
pub const SPRST_NUM_STATES: i32 = 4i32;
pub const SPRP_NORMAL: u32 = 0u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SPRULE {
    pub pszRuleName: super::super::Foundation::PWSTR,
    pub ulRuleId: u32,
    pub dwAttributes: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPRULE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPRULE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPRS_INACTIVE: i32 = 0i32;
pub const SPRS_ACTIVE: i32 = 1i32;
pub const SPRS_ACTIVE_WITH_AUTO_PAUSE: i32 = 3i32;
pub const SPRS_ACTIVE_USER_DELIMITED: i32 = 4i32;
pub const SPRS_DONE: i32 = 1i32;
pub const SPRS_IS_SPEAKING: i32 = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SPSEMANTICERRORINFO {
    pub ulLineNumber: u32,
    pub pszScriptLine: super::super::Foundation::PWSTR,
    pub pszSource: super::super::Foundation::PWSTR,
    pub pszDescription: super::super::Foundation::PWSTR,
    pub hrResultCode: ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPSEMANTICERRORINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPSEMANTICERRORINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPSMF_SAPI_PROPERTIES: i32 = 0i32;
pub const SPSMF_SRGS_SEMANTICINTERPRETATION_MS: i32 = 1i32;
pub const SPSMF_SRGS_SAPIPROPERTIES: i32 = 2i32;
pub const SPSMF_UPS: i32 = 4i32;
pub const SPSMF_SRGS_SEMANTICINTERPRETATION_W3C: i32 = 8i32;
#[repr(C)]
pub struct SPSERIALIZEDEVENT {
    pub _bitfield: i32,
    pub ulStreamNum: u32,
    pub ullAudioStreamOffset: u64,
    pub SerializedwParam: u32,
    pub SerializedlParam: i32,
}
impl ::core::marker::Copy for SPSERIALIZEDEVENT {}
impl ::core::clone::Clone for SPSERIALIZEDEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SPSERIALIZEDEVENT64 {
    pub _bitfield: i32,
    pub ulStreamNum: u32,
    pub ullAudioStreamOffset: u64,
    pub SerializedwParam: u64,
    pub SerializedlParam: i64,
}
impl ::core::marker::Copy for SPSERIALIZEDEVENT64 {}
impl ::core::clone::Clone for SPSERIALIZEDEVENT64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SPSERIALIZEDPHRASE {
    pub ulSerializedSize: u32,
}
impl ::core::marker::Copy for SPSERIALIZEDPHRASE {}
impl ::core::clone::Clone for SPSERIALIZEDPHRASE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SPSERIALIZEDRESULT {
    pub ulSerializedSize: u32,
}
impl ::core::marker::Copy for SPSERIALIZEDRESULT {}
impl ::core::clone::Clone for SPSERIALIZEDRESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SPSHORTCUTPAIR {
    pub pNextSHORTCUTPAIR: *mut SPSHORTCUTPAIR,
    pub LangID: u16,
    pub shType: SPSHORTCUTTYPE,
    pub pszDisplay: super::super::Foundation::PWSTR,
    pub pszSpoken: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPSHORTCUTPAIR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPSHORTCUTPAIR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SPSHORTCUTPAIRLIST {
    pub ulSize: u32,
    pub pvBuffer: *mut u8,
    pub pFirstShortcutPair: *mut SPSHORTCUTPAIR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPSHORTCUTPAIRLIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPSHORTCUTPAIRLIST {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPSHT_NotOverriden: i32 = -1i32;
pub const SPSHT_Unknown: i32 = 0i32;
pub const SPSHT_EMAIL: i32 = 4096i32;
pub const SPSHT_OTHER: i32 = 8192i32;
pub const SPPS_RESERVED1: i32 = 12288i32;
pub const SPPS_RESERVED2: i32 = 16384i32;
pub const SPPS_RESERVED3: i32 = 20480i32;
pub const SPPS_RESERVED4: i32 = 61440i32;
#[repr(C)]
pub struct SPSTATEHANDLE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for SPSTATEHANDLE__ {}
impl ::core::clone::Clone for SPSTATEHANDLE__ {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPSF_Default: i32 = -1i32;
pub const SPSF_NoAssignedFormat: i32 = 0i32;
pub const SPSF_Text: i32 = 1i32;
pub const SPSF_NonStandardFormat: i32 = 2i32;
pub const SPSF_ExtendedAudioFormat: i32 = 3i32;
pub const SPSF_8kHz8BitMono: i32 = 4i32;
pub const SPSF_8kHz8BitStereo: i32 = 5i32;
pub const SPSF_8kHz16BitMono: i32 = 6i32;
pub const SPSF_8kHz16BitStereo: i32 = 7i32;
pub const SPSF_11kHz8BitMono: i32 = 8i32;
pub const SPSF_11kHz8BitStereo: i32 = 9i32;
pub const SPSF_11kHz16BitMono: i32 = 10i32;
pub const SPSF_11kHz16BitStereo: i32 = 11i32;
pub const SPSF_12kHz8BitMono: i32 = 12i32;
pub const SPSF_12kHz8BitStereo: i32 = 13i32;
pub const SPSF_12kHz16BitMono: i32 = 14i32;
pub const SPSF_12kHz16BitStereo: i32 = 15i32;
pub const SPSF_16kHz8BitMono: i32 = 16i32;
pub const SPSF_16kHz8BitStereo: i32 = 17i32;
pub const SPSF_16kHz16BitMono: i32 = 18i32;
pub const SPSF_16kHz16BitStereo: i32 = 19i32;
pub const SPSF_22kHz8BitMono: i32 = 20i32;
pub const SPSF_22kHz8BitStereo: i32 = 21i32;
pub const SPSF_22kHz16BitMono: i32 = 22i32;
pub const SPSF_22kHz16BitStereo: i32 = 23i32;
pub const SPSF_24kHz8BitMono: i32 = 24i32;
pub const SPSF_24kHz8BitStereo: i32 = 25i32;
pub const SPSF_24kHz16BitMono: i32 = 26i32;
pub const SPSF_24kHz16BitStereo: i32 = 27i32;
pub const SPSF_32kHz8BitMono: i32 = 28i32;
pub const SPSF_32kHz8BitStereo: i32 = 29i32;
pub const SPSF_32kHz16BitMono: i32 = 30i32;
pub const SPSF_32kHz16BitStereo: i32 = 31i32;
pub const SPSF_44kHz8BitMono: i32 = 32i32;
pub const SPSF_44kHz8BitStereo: i32 = 33i32;
pub const SPSF_44kHz16BitMono: i32 = 34i32;
pub const SPSF_44kHz16BitStereo: i32 = 35i32;
pub const SPSF_48kHz8BitMono: i32 = 36i32;
pub const SPSF_48kHz8BitStereo: i32 = 37i32;
pub const SPSF_48kHz16BitMono: i32 = 38i32;
pub const SPSF_48kHz16BitStereo: i32 = 39i32;
pub const SPSF_TrueSpeech_8kHz1BitMono: i32 = 40i32;
pub const SPSF_CCITT_ALaw_8kHzMono: i32 = 41i32;
pub const SPSF_CCITT_ALaw_8kHzStereo: i32 = 42i32;
pub const SPSF_CCITT_ALaw_11kHzMono: i32 = 43i32;
pub const SPSF_CCITT_ALaw_11kHzStereo: i32 = 44i32;
pub const SPSF_CCITT_ALaw_22kHzMono: i32 = 45i32;
pub const SPSF_CCITT_ALaw_22kHzStereo: i32 = 46i32;
pub const SPSF_CCITT_ALaw_44kHzMono: i32 = 47i32;
pub const SPSF_CCITT_ALaw_44kHzStereo: i32 = 48i32;
pub const SPSF_CCITT_uLaw_8kHzMono: i32 = 49i32;
pub const SPSF_CCITT_uLaw_8kHzStereo: i32 = 50i32;
pub const SPSF_CCITT_uLaw_11kHzMono: i32 = 51i32;
pub const SPSF_CCITT_uLaw_11kHzStereo: i32 = 52i32;
pub const SPSF_CCITT_uLaw_22kHzMono: i32 = 53i32;
pub const SPSF_CCITT_uLaw_22kHzStereo: i32 = 54i32;
pub const SPSF_CCITT_uLaw_44kHzMono: i32 = 55i32;
pub const SPSF_CCITT_uLaw_44kHzStereo: i32 = 56i32;
pub const SPSF_ADPCM_8kHzMono: i32 = 57i32;
pub const SPSF_ADPCM_8kHzStereo: i32 = 58i32;
pub const SPSF_ADPCM_11kHzMono: i32 = 59i32;
pub const SPSF_ADPCM_11kHzStereo: i32 = 60i32;
pub const SPSF_ADPCM_22kHzMono: i32 = 61i32;
pub const SPSF_ADPCM_22kHzStereo: i32 = 62i32;
pub const SPSF_ADPCM_44kHzMono: i32 = 63i32;
pub const SPSF_ADPCM_44kHzStereo: i32 = 64i32;
pub const SPSF_GSM610_8kHzMono: i32 = 65i32;
pub const SPSF_GSM610_11kHzMono: i32 = 66i32;
pub const SPSF_GSM610_22kHzMono: i32 = 67i32;
pub const SPSF_GSM610_44kHzMono: i32 = 68i32;
pub const SPSF_NUM_FORMATS: i32 = 69i32;
#[repr(C)]
pub struct SPTEXTSELECTIONINFO {
    pub ulStartActiveOffset: u32,
    pub cchActiveChars: u32,
    pub ulStartSelection: u32,
    pub cchSelection: u32,
}
impl ::core::marker::Copy for SPTEXTSELECTIONINFO {}
impl ::core::clone::Clone for SPTEXTSELECTIONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPVA_Speak: i32 = 0i32;
pub const SPVA_Silence: i32 = 1i32;
pub const SPVA_Pronounce: i32 = 2i32;
pub const SPVA_Bookmark: i32 = 3i32;
pub const SPVA_SpellOut: i32 = 4i32;
pub const SPVA_Section: i32 = 5i32;
pub const SPVA_ParseUnknownTag: i32 = 6i32;
pub const SPDF_PROPERTY: i32 = 1i32;
pub const SPDF_REPLACEMENT: i32 = 2i32;
pub const SPDF_RULE: i32 = 4i32;
pub const SPDF_DISPLAYTEXT: i32 = 8i32;
pub const SPDF_LEXICALFORM: i32 = 16i32;
pub const SPDF_PRONUNCIATION: i32 = 32i32;
pub const SPDF_AUDIO: i32 = 64i32;
pub const SPDF_ALTERNATES: i32 = 128i32;
pub const SPDF_ALL: i32 = 255i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SPVCONTEXT {
    pub pCategory: super::super::Foundation::PWSTR,
    pub pBefore: super::super::Foundation::PWSTR,
    pub pAfter: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPVCONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPVCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPVFEATURE_STRESSED: i32 = 1i32;
pub const SPVFEATURE_EMPHASIS: i32 = 2i32;
pub const SP_VISEME_0: i32 = 0i32;
pub const SP_VISEME_1: i32 = 1i32;
pub const SP_VISEME_2: i32 = 2i32;
pub const SP_VISEME_3: i32 = 3i32;
pub const SP_VISEME_4: i32 = 4i32;
pub const SP_VISEME_5: i32 = 5i32;
pub const SP_VISEME_6: i32 = 6i32;
pub const SP_VISEME_7: i32 = 7i32;
pub const SP_VISEME_8: i32 = 8i32;
pub const SP_VISEME_9: i32 = 9i32;
pub const SP_VISEME_10: i32 = 10i32;
pub const SP_VISEME_11: i32 = 11i32;
pub const SP_VISEME_12: i32 = 12i32;
pub const SP_VISEME_13: i32 = 13i32;
pub const SP_VISEME_14: i32 = 14i32;
pub const SP_VISEME_15: i32 = 15i32;
pub const SP_VISEME_16: i32 = 16i32;
pub const SP_VISEME_17: i32 = 17i32;
pub const SP_VISEME_18: i32 = 18i32;
pub const SP_VISEME_19: i32 = 19i32;
pub const SP_VISEME_20: i32 = 20i32;
pub const SP_VISEME_21: i32 = 21i32;
pub const SPMIN_VOLUME: i32 = 0i32;
pub const SPMAX_VOLUME: i32 = 100i32;
pub const SPMIN_RATE: i32 = -10i32;
pub const SPMAX_RATE: i32 = 10i32;
#[repr(C)]
pub struct SPVOICESTATUS {
    pub ulCurrentStream: u32,
    pub ulLastStreamQueued: u32,
    pub hrLastResult: ::windows_sys::core::HRESULT,
    pub dwRunningState: u32,
    pub ulInputWordPos: u32,
    pub ulInputWordLen: u32,
    pub ulInputSentPos: u32,
    pub ulInputSentLen: u32,
    pub lBookmarkId: i32,
    pub PhonemeId: u16,
    pub VisemeId: SPVISEMES,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for SPVOICESTATUS {}
impl ::core::clone::Clone for SPVOICESTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SPVPITCH {
    pub MiddleAdj: i32,
    pub RangeAdj: i32,
}
impl ::core::marker::Copy for SPVPITCH {}
impl ::core::clone::Clone for SPVPITCH {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPVPRI_NORMAL: i32 = 0i32;
pub const SPVPRI_ALERT: i32 = 1i32;
pub const SPVPRI_OVER: i32 = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SPVSTATE {
    pub eAction: SPVACTIONS,
    pub LangID: u16,
    pub wReserved: u16,
    pub EmphAdj: i32,
    pub RateAdj: i32,
    pub Volume: u32,
    pub PitchAdj: SPVPITCH,
    pub SilenceMSecs: u32,
    pub pPhoneIds: *mut u16,
    pub ePartOfSpeech: SPPARTOFSPEECH,
    pub Context: SPVCONTEXT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPVSTATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPVSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPWF_INPUT: i32 = 0i32;
pub const SPWF_SRENGINE: i32 = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SPWORD {
    pub pNextWord: *mut SPWORD,
    pub LangID: u16,
    pub wReserved: u16,
    pub eWordType: SPWORDTYPE,
    pub pszWord: super::super::Foundation::PWSTR,
    pub pFirstWordPronunciation: *mut SPWORDPRONUNCIATION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPWORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPWORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SPWORDLIST {
    pub ulSize: u32,
    pub pvBuffer: *mut u8,
    pub pFirstWord: *mut SPWORD,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPWORDLIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPWORDLIST {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPWP_UNKNOWN_WORD_UNPRONOUNCEABLE: i32 = 0i32;
pub const SPWP_UNKNOWN_WORD_PRONOUNCEABLE: i32 = 1i32;
pub const SPWP_KNOWN_WORD_PRONOUNCEABLE: i32 = 2i32;
#[repr(C)]
pub struct SPWORDPRONUNCIATION {
    pub pNextWordPronunciation: *mut SPWORDPRONUNCIATION,
    pub eLexiconType: SPLEXICONTYPE,
    pub LangID: u16,
    pub wPronunciationFlags: u16,
    pub ePartOfSpeech: SPPARTOFSPEECH,
    pub szPronunciation: [u16; 1],
}
impl ::core::marker::Copy for SPWORDPRONUNCIATION {}
impl ::core::clone::Clone for SPWORDPRONUNCIATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SPWORDPRONUNCIATIONLIST {
    pub ulSize: u32,
    pub pvBuffer: *mut u8,
    pub pFirstWordPronunciation: *mut SPWORDPRONUNCIATION,
}
impl ::core::marker::Copy for SPWORDPRONUNCIATIONLIST {}
impl ::core::clone::Clone for SPWORDPRONUNCIATIONLIST {
    fn clone(&self) -> Self {
        *self
    }
}
pub const eWORDTYPE_ADDED: i32 = 1i32;
pub const eWORDTYPE_DELETED: i32 = 2i32;
pub const SPXRO_SML: i32 = 0i32;
pub const SPXRO_Alternates_SML: i32 = 1i32;
pub const SP_EMULATE_RESULT: u32 = 1073741824u32;
pub const SP_LOW_CONFIDENCE: i32 = -1i32;
pub const SP_MAX_LANGIDS: u32 = 20u32;
pub const SP_MAX_PRON_LENGTH: u32 = 384u32;
pub const SP_MAX_WORD_LENGTH: u32 = 128u32;
pub const SP_NORMAL_CONFIDENCE: u32 = 0u32;
pub const SP_STREAMPOS_ASAP: u32 = 0u32;
pub const SP_STREAMPOS_REALTIME: i32 = -1i32;
pub const SpAudioFormat: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2667145328, data2: 57696, data3: 18322, data4: [130, 13, 72, 207, 6, 73, 228, 236] };
pub const SpCompressedLexicon: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2425370390, data2: 12098, data3: 4563, data4: [156, 38, 0, 192, 79, 142, 248, 124] };
pub const SpCustomStream: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2378101055,
    data2: 6472,
    data3: 19112,
    data4: [140, 240, 4, 142, 235, 237, 149, 216],
};
pub const SpFileStream: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2490897075,
    data2: 10977,
    data3: 17988,
    data4: [186, 134, 158, 144, 222, 215, 236, 145],
};
pub const SpInProcRecoContext: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1940744258,
    data2: 44256,
    data3: 17896,
    data4: [164, 221, 135, 149, 136, 26, 44, 42],
};
pub const SpInprocRecognizer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1102617451, data2: 37785, data3: 4562, data4: [150, 35, 0, 192, 79, 142, 230, 40] };
pub const SpLexicon: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 106292118, data2: 9680, data3: 4563, data4: [156, 38, 0, 192, 79, 142, 248, 124] };
pub const SpMMAudioEnum: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2870513824, data2: 59679, data3: 4562, data4: [187, 145, 0, 192, 79, 142, 230, 192] };
pub const SpMMAudioIn: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3476893264, data2: 21490, data3: 4562, data4: [150, 12, 0, 192, 79, 142, 230, 40] };
pub const SpMMAudioOut: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2831581419, data2: 15666, data3: 4562, data4: [158, 231, 0, 192, 79, 121, 115, 150] };
pub const SpMemoryStream: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1605889917,
    data2: 57332,
    data3: 18058,
    data4: [182, 183, 47, 203, 209, 136, 249, 148],
};
pub const SpNotifyTranslator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3803075442, data2: 23872, data3: 4562, data4: [150, 14, 0, 192, 79, 142, 230, 40] };
pub const SpNullPhoneConverter: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1163863273,
    data2: 29590,
    data3: 18966,
    data4: [151, 21, 124, 15, 219, 227, 239, 227],
};
pub const SpObjectToken: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4014020434,
    data2: 14134,
    data3: 19636,
    data4: [156, 140, 142, 244, 204, 181, 142, 254],
};
pub const SpObjectTokenCategory: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2836404351,
    data2: 3194,
    data3: 17836,
    data4: [146, 204, 89, 237, 175, 183, 123, 83],
};
pub const SpPhoneConverter: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2441475907,
    data2: 4419,
    data3: 19496,
    data4: [134, 181, 191, 241, 79, 32, 229, 200],
};
pub const SpPhoneticAlphabetConverter: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1329676582,
    data2: 57315,
    data3: 17961,
    data4: [153, 238, 121, 121, 120, 49, 126, 173],
};
pub const SpPhraseInfoBuilder: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3258958477,
    data2: 50527,
    data3: 18208,
    data4: [139, 50, 145, 247, 60, 43, 213, 209],
};
pub const SpResourceManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2524222323, data2: 13201, data3: 4562, data4: [158, 227, 0, 192, 79, 121, 115, 150] };
pub const SpSharedRecoContext: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1193304580, data2: 24266, data3: 4562, data4: [150, 15, 0, 192, 79, 142, 230, 40] };
pub const SpSharedRecognizer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1005471888, data2: 20457, data3: 18999, data4: [140, 30, 94, 126, 18, 121, 28, 31] };
pub const SpShortcut: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 225586970,
    data2: 40911,
    data3: 20066,
    data4: [150, 216, 109, 248, 240, 26, 38, 170],
};
pub const SpStream: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1901960281, data2: 17474, data3: 4562, data4: [150, 5, 0, 192, 79, 142, 230, 40] };
pub const SpStreamFormatConverter: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1880331322, data2: 58092, data3: 4562, data4: [160, 134, 0, 192, 79, 142, 249, 181] };
pub const SpTextSelectionInformation: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 261227274,
    data2: 52221,
    data3: 19128,
    data4: [161, 100, 255, 89, 133, 84, 127, 246],
};
pub const SpUnCompressedLexicon: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3387128853,
    data2: 57234,
    data3: 18215,
    data4: [133, 214, 114, 229, 238, 182, 153, 90],
};
pub const SpVoice: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2524222327, data2: 13201, data3: 4562, data4: [158, 227, 0, 192, 79, 121, 115, 150] };
pub const SpWaveFormatEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3348780876,
    data2: 25534,
    data3: 17593,
    data4: [128, 31, 40, 63, 135, 248, 152, 190],
};
pub const SpeechAllElements: i32 = -1i32;
pub const SAFTDefault: i32 = -1i32;
pub const SAFTNoAssignedFormat: i32 = 0i32;
pub const SAFTText: i32 = 1i32;
pub const SAFTNonStandardFormat: i32 = 2i32;
pub const SAFTExtendedAudioFormat: i32 = 3i32;
pub const SAFT8kHz8BitMono: i32 = 4i32;
pub const SAFT8kHz8BitStereo: i32 = 5i32;
pub const SAFT8kHz16BitMono: i32 = 6i32;
pub const SAFT8kHz16BitStereo: i32 = 7i32;
pub const SAFT11kHz8BitMono: i32 = 8i32;
pub const SAFT11kHz8BitStereo: i32 = 9i32;
pub const SAFT11kHz16BitMono: i32 = 10i32;
pub const SAFT11kHz16BitStereo: i32 = 11i32;
pub const SAFT12kHz8BitMono: i32 = 12i32;
pub const SAFT12kHz8BitStereo: i32 = 13i32;
pub const SAFT12kHz16BitMono: i32 = 14i32;
pub const SAFT12kHz16BitStereo: i32 = 15i32;
pub const SAFT16kHz8BitMono: i32 = 16i32;
pub const SAFT16kHz8BitStereo: i32 = 17i32;
pub const SAFT16kHz16BitMono: i32 = 18i32;
pub const SAFT16kHz16BitStereo: i32 = 19i32;
pub const SAFT22kHz8BitMono: i32 = 20i32;
pub const SAFT22kHz8BitStereo: i32 = 21i32;
pub const SAFT22kHz16BitMono: i32 = 22i32;
pub const SAFT22kHz16BitStereo: i32 = 23i32;
pub const SAFT24kHz8BitMono: i32 = 24i32;
pub const SAFT24kHz8BitStereo: i32 = 25i32;
pub const SAFT24kHz16BitMono: i32 = 26i32;
pub const SAFT24kHz16BitStereo: i32 = 27i32;
pub const SAFT32kHz8BitMono: i32 = 28i32;
pub const SAFT32kHz8BitStereo: i32 = 29i32;
pub const SAFT32kHz16BitMono: i32 = 30i32;
pub const SAFT32kHz16BitStereo: i32 = 31i32;
pub const SAFT44kHz8BitMono: i32 = 32i32;
pub const SAFT44kHz8BitStereo: i32 = 33i32;
pub const SAFT44kHz16BitMono: i32 = 34i32;
pub const SAFT44kHz16BitStereo: i32 = 35i32;
pub const SAFT48kHz8BitMono: i32 = 36i32;
pub const SAFT48kHz8BitStereo: i32 = 37i32;
pub const SAFT48kHz16BitMono: i32 = 38i32;
pub const SAFT48kHz16BitStereo: i32 = 39i32;
pub const SAFTTrueSpeech_8kHz1BitMono: i32 = 40i32;
pub const SAFTCCITT_ALaw_8kHzMono: i32 = 41i32;
pub const SAFTCCITT_ALaw_8kHzStereo: i32 = 42i32;
pub const SAFTCCITT_ALaw_11kHzMono: i32 = 43i32;
pub const SAFTCCITT_ALaw_11kHzStereo: i32 = 44i32;
pub const SAFTCCITT_ALaw_22kHzMono: i32 = 45i32;
pub const SAFTCCITT_ALaw_22kHzStereo: i32 = 46i32;
pub const SAFTCCITT_ALaw_44kHzMono: i32 = 47i32;
pub const SAFTCCITT_ALaw_44kHzStereo: i32 = 48i32;
pub const SAFTCCITT_uLaw_8kHzMono: i32 = 49i32;
pub const SAFTCCITT_uLaw_8kHzStereo: i32 = 50i32;
pub const SAFTCCITT_uLaw_11kHzMono: i32 = 51i32;
pub const SAFTCCITT_uLaw_11kHzStereo: i32 = 52i32;
pub const SAFTCCITT_uLaw_22kHzMono: i32 = 53i32;
pub const SAFTCCITT_uLaw_22kHzStereo: i32 = 54i32;
pub const SAFTCCITT_uLaw_44kHzMono: i32 = 55i32;
pub const SAFTCCITT_uLaw_44kHzStereo: i32 = 56i32;
pub const SAFTADPCM_8kHzMono: i32 = 57i32;
pub const SAFTADPCM_8kHzStereo: i32 = 58i32;
pub const SAFTADPCM_11kHzMono: i32 = 59i32;
pub const SAFTADPCM_11kHzStereo: i32 = 60i32;
pub const SAFTADPCM_22kHzMono: i32 = 61i32;
pub const SAFTADPCM_22kHzStereo: i32 = 62i32;
pub const SAFTADPCM_44kHzMono: i32 = 63i32;
pub const SAFTADPCM_44kHzStereo: i32 = 64i32;
pub const SAFTGSM610_8kHzMono: i32 = 65i32;
pub const SAFTGSM610_11kHzMono: i32 = 66i32;
pub const SAFTGSM610_22kHzMono: i32 = 67i32;
pub const SAFTGSM610_44kHzMono: i32 = 68i32;
pub const SASClosed: i32 = 0i32;
pub const SASStop: i32 = 1i32;
pub const SASPause: i32 = 2i32;
pub const SASRun: i32 = 3i32;
pub const SBONone: i32 = 0i32;
pub const SBOPause: i32 = 1i32;
pub const SDKLDefaultLocation: i32 = 0i32;
pub const SDKLCurrentUser: i32 = 1i32;
pub const SDKLLocalMachine: i32 = 2i32;
pub const SDKLCurrentConfig: i32 = 5i32;
pub const SDTProperty: i32 = 1i32;
pub const SDTReplacement: i32 = 2i32;
pub const SDTRule: i32 = 4i32;
pub const SDTDisplayText: i32 = 8i32;
pub const SDTLexicalForm: i32 = 16i32;
pub const SDTPronunciation: i32 = 32i32;
pub const SDTAudio: i32 = 64i32;
pub const SDTAlternates: i32 = 128i32;
pub const SDTAll: i32 = 255i32;
pub const SDA_No_Trailing_Space: i32 = 0i32;
pub const SDA_One_Trailing_Space: i32 = 2i32;
pub const SDA_Two_Trailing_Spaces: i32 = 4i32;
pub const SDA_Consume_Leading_Spaces: i32 = 8i32;
pub const SECFIgnoreCase: i32 = 1i32;
pub const SECFIgnoreKanaType: i32 = 65536i32;
pub const SECFIgnoreWidth: i32 = 131072i32;
pub const SECFNoSpecialChars: i32 = 536870912i32;
pub const SECFEmulateResult: i32 = 1073741824i32;
pub const SECFDefault: i32 = 196609i32;
pub const SECLowConfidence: i32 = -1i32;
pub const SECNormalConfidence: i32 = 0i32;
pub const SECHighConfidence: i32 = 1i32;
pub const SFTInput: i32 = 0i32;
pub const SFTSREngine: i32 = 1i32;
pub const SGRSTTEpsilon: i32 = 0i32;
pub const SGRSTTWord: i32 = 1i32;
pub const SGRSTTRule: i32 = 2i32;
pub const SGRSTTDictation: i32 = 3i32;
pub const SGRSTTWildcard: i32 = 4i32;
pub const SGRSTTTextBuffer: i32 = 5i32;
pub const SGSEnabled: i32 = 1i32;
pub const SGSDisabled: i32 = 0i32;
pub const SGSExclusive: i32 = 3i32;
pub const SGDisplay: i32 = 0i32;
pub const SGLexical: i32 = 1i32;
pub const SGPronounciation: i32 = 2i32;
pub const SGLexicalNoSpecialChars: i32 = 3i32;
pub const SINone: i32 = 0i32;
pub const SINoise: i32 = 1i32;
pub const SINoSignal: i32 = 2i32;
pub const SITooLoud: i32 = 3i32;
pub const SITooQuiet: i32 = 4i32;
pub const SITooFast: i32 = 5i32;
pub const SITooSlow: i32 = 6i32;
pub const SLTUser: i32 = 1i32;
pub const SLTApp: i32 = 2i32;
pub const SLOStatic: i32 = 0i32;
pub const SLODynamic: i32 = 1i32;
pub const SPSNotOverriden: i32 = -1i32;
pub const SPSUnknown: i32 = 0i32;
pub const SPSNoun: i32 = 4096i32;
pub const SPSVerb: i32 = 8192i32;
pub const SPSModifier: i32 = 12288i32;
pub const SPSFunction: i32 = 16384i32;
pub const SPSInterjection: i32 = 20480i32;
pub const SPSLMA: i32 = 28672i32;
pub const SPSSuppressWord: i32 = 61440i32;
pub const SRCS_Disabled: i32 = 0i32;
pub const SRCS_Enabled: i32 = 1i32;
pub const SREStreamEnd: i32 = 1i32;
pub const SRESoundStart: i32 = 2i32;
pub const SRESoundEnd: i32 = 4i32;
pub const SREPhraseStart: i32 = 8i32;
pub const SRERecognition: i32 = 16i32;
pub const SREHypothesis: i32 = 32i32;
pub const SREBookmark: i32 = 64i32;
pub const SREPropertyNumChange: i32 = 128i32;
pub const SREPropertyStringChange: i32 = 256i32;
pub const SREFalseRecognition: i32 = 512i32;
pub const SREInterference: i32 = 1024i32;
pub const SRERequestUI: i32 = 2048i32;
pub const SREStateChange: i32 = 4096i32;
pub const SREAdaptation: i32 = 8192i32;
pub const SREStreamStart: i32 = 16384i32;
pub const SRERecoOtherContext: i32 = 32768i32;
pub const SREAudioLevel: i32 = 65536i32;
pub const SREPrivate: i32 = 262144i32;
pub const SREAllEvents: i32 = 393215i32;
pub const SRTStandard: i32 = 0i32;
pub const SRTAutopause: i32 = 1i32;
pub const SRTEmulated: i32 = 2i32;
pub const SRTSMLTimeout: i32 = 4i32;
pub const SRTExtendableParse: i32 = 8i32;
pub const SRTReSent: i32 = 16i32;
pub const SRSInactive: i32 = 0i32;
pub const SRSActive: i32 = 1i32;
pub const SRSActiveAlways: i32 = 2i32;
pub const SRSInactiveWithPurge: i32 = 3i32;
pub const SRAONone: i32 = 0i32;
pub const SRAORetainAudio: i32 = 1i32;
pub const SRATopLevel: i32 = 1i32;
pub const SRADefaultToActive: i32 = 2i32;
pub const SRAExport: i32 = 4i32;
pub const SRAImport: i32 = 8i32;
pub const SRAInterpreter: i32 = 16i32;
pub const SRADynamic: i32 = 32i32;
pub const SRARoot: i32 = 64i32;
pub const SGDSInactive: i32 = 0i32;
pub const SGDSActive: i32 = 1i32;
pub const SGDSActiveWithAutoPause: i32 = 3i32;
pub const SGDSActiveUserDelimited: i32 = 4i32;
pub const SRSEDone: i32 = 1i32;
pub const SRSEIsSpeaking: i32 = 2i32;
pub const SSTTWildcard: i32 = 1i32;
pub const SSTTDictation: i32 = 2i32;
pub const SSTTTextBuffer: i32 = 3i32;
pub const SSFMOpenForRead: i32 = 0i32;
pub const SSFMOpenReadWrite: i32 = 1i32;
pub const SSFMCreate: i32 = 2i32;
pub const SSFMCreateForWrite: i32 = 3i32;
pub const SSSPTRelativeToStart: u32 = 0u32;
pub const SSSPTRelativeToCurrentPosition: u32 = 1u32;
pub const SSSPTRelativeToEnd: u32 = 2u32;
pub const STCInprocServer: u32 = 1u32;
pub const STCInprocHandler: u32 = 2u32;
pub const STCLocalServer: u32 = 4u32;
pub const STCRemoteServer: u32 = 16u32;
pub const STCAll: u32 = 23u32;
pub const STSF_AppData: i32 = 26i32;
pub const STSF_LocalAppData: i32 = 28i32;
pub const STSF_CommonAppData: i32 = 35i32;
pub const STSF_FlagCreate: i32 = 32768i32;
pub const SVF_None: i32 = 0i32;
pub const SVF_Stressed: i32 = 1i32;
pub const SVF_Emphasis: i32 = 2i32;
pub const SVP_0: i32 = 0i32;
pub const SVP_1: i32 = 1i32;
pub const SVP_2: i32 = 2i32;
pub const SVP_3: i32 = 3i32;
pub const SVP_4: i32 = 4i32;
pub const SVP_5: i32 = 5i32;
pub const SVP_6: i32 = 6i32;
pub const SVP_7: i32 = 7i32;
pub const SVP_8: i32 = 8i32;
pub const SVP_9: i32 = 9i32;
pub const SVP_10: i32 = 10i32;
pub const SVP_11: i32 = 11i32;
pub const SVP_12: i32 = 12i32;
pub const SVP_13: i32 = 13i32;
pub const SVP_14: i32 = 14i32;
pub const SVP_15: i32 = 15i32;
pub const SVP_16: i32 = 16i32;
pub const SVP_17: i32 = 17i32;
pub const SVP_18: i32 = 18i32;
pub const SVP_19: i32 = 19i32;
pub const SVP_20: i32 = 20i32;
pub const SVP_21: i32 = 21i32;
pub const SVEStartInputStream: i32 = 2i32;
pub const SVEEndInputStream: i32 = 4i32;
pub const SVEVoiceChange: i32 = 8i32;
pub const SVEBookmark: i32 = 16i32;
pub const SVEWordBoundary: i32 = 32i32;
pub const SVEPhoneme: i32 = 64i32;
pub const SVESentenceBoundary: i32 = 128i32;
pub const SVEViseme: i32 = 256i32;
pub const SVEAudioLevel: i32 = 512i32;
pub const SVEPrivate: i32 = 32768i32;
pub const SVEAllEvents: i32 = 33790i32;
pub const SVPNormal: i32 = 0i32;
pub const SVPAlert: i32 = 1i32;
pub const SVPOver: i32 = 2i32;
pub const SVSFDefault: i32 = 0i32;
pub const SVSFlagsAsync: i32 = 1i32;
pub const SVSFPurgeBeforeSpeak: i32 = 2i32;
pub const SVSFIsFilename: i32 = 4i32;
pub const SVSFIsXML: i32 = 8i32;
pub const SVSFIsNotXML: i32 = 16i32;
pub const SVSFPersistXML: i32 = 32i32;
pub const SVSFNLPSpeakPunc: i32 = 64i32;
pub const SVSFParseSapi: i32 = 128i32;
pub const SVSFParseSsml: i32 = 256i32;
pub const SVSFParseAutodetect: i32 = 0i32;
pub const SVSFNLPMask: i32 = 64i32;
pub const SVSFParseMask: i32 = 384i32;
pub const SVSFVoiceMask: i32 = 511i32;
pub const SVSFUnusedFlags: i32 = -512i32;
pub const SWPUnknownWordUnpronounceable: i32 = 0i32;
pub const SWPUnknownWordPronounceable: i32 = 1i32;
pub const SWPKnownWordPronounceable: i32 = 2i32;
pub const SWTAdded: i32 = 1i32;
pub const SWTDeleted: i32 = 2i32;
pub const Speech_Default_Weight: f32 = 1f32;
pub const Speech_Max_Pron_Length: i32 = 384i32;
pub const Speech_Max_Word_Length: i32 = 128i32;
pub const Speech_StreamPos_Asap: i32 = 0i32;
pub const Speech_StreamPos_RealTime: i32 = -1i32;
#[repr(transparent)]
pub struct _ISpeechRecoContextEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for _ISpeechRecoContextEvents {}
impl ::core::clone::Clone for _ISpeechRecoContextEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct _ISpeechVoiceEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for _ISpeechVoiceEvents {}
impl ::core::clone::Clone for _ISpeechVoiceEvents {
    fn clone(&self) -> Self {
        *self
    }
}
