pub const AllWords: SPMATCHINGMODE = 0;
pub const DEFAULT_WEIGHT: u32 = 1;
pub type DISPIDSPRG = i32;
pub type DISPIDSPTSI = i32;
pub const DISPIDSPTSI_ActiveLength: DISPIDSPTSI = 2;
pub const DISPIDSPTSI_ActiveOffset: DISPIDSPTSI = 1;
pub const DISPIDSPTSI_SelectionLength: DISPIDSPTSI = 4;
pub const DISPIDSPTSI_SelectionOffset: DISPIDSPTSI = 3;
pub const DISPID_SABIBufferSize: DISPID_SpeechAudioBufferInfo = 2;
pub const DISPID_SABIEventBias: DISPID_SpeechAudioBufferInfo = 3;
pub const DISPID_SABIMinNotification: DISPID_SpeechAudioBufferInfo = 1;
pub const DISPID_SABufferInfo: DISPID_SpeechAudio = 201;
pub const DISPID_SABufferNotifySize: DISPID_SpeechAudio = 204;
pub const DISPID_SADefaultFormat: DISPID_SpeechAudio = 202;
pub const DISPID_SAEventHandle: DISPID_SpeechAudio = 205;
pub const DISPID_SAFGetWaveFormatEx: DISPID_SpeechAudioFormat = 3;
pub const DISPID_SAFGuid: DISPID_SpeechAudioFormat = 2;
pub const DISPID_SAFSetWaveFormatEx: DISPID_SpeechAudioFormat = 4;
pub const DISPID_SAFType: DISPID_SpeechAudioFormat = 1;
pub const DISPID_SASCurrentDevicePosition: DISPID_SpeechAudioStatus = 5;
pub const DISPID_SASCurrentSeekPosition: DISPID_SpeechAudioStatus = 4;
pub const DISPID_SASFreeBufferSpace: DISPID_SpeechAudioStatus = 1;
pub const DISPID_SASNonBlockingIO: DISPID_SpeechAudioStatus = 2;
pub const DISPID_SASState: DISPID_SpeechAudioStatus = 3;
pub const DISPID_SASetState: DISPID_SpeechAudio = 206;
pub const DISPID_SAStatus: DISPID_SpeechAudio = 200;
pub const DISPID_SAVolume: DISPID_SpeechAudio = 203;
pub const DISPID_SBSFormat: DISPID_SpeechBaseStream = 1;
pub const DISPID_SBSRead: DISPID_SpeechBaseStream = 2;
pub const DISPID_SBSSeek: DISPID_SpeechBaseStream = 4;
pub const DISPID_SBSWrite: DISPID_SpeechBaseStream = 3;
pub const DISPID_SCSBaseStream: DISPID_SpeechCustomStream = 100;
pub const DISPID_SDKCreateKey: DISPID_SpeechDataKey = 8;
pub const DISPID_SDKDeleteKey: DISPID_SpeechDataKey = 9;
pub const DISPID_SDKDeleteValue: DISPID_SpeechDataKey = 10;
pub const DISPID_SDKEnumKeys: DISPID_SpeechDataKey = 11;
pub const DISPID_SDKEnumValues: DISPID_SpeechDataKey = 12;
pub const DISPID_SDKGetBinaryValue: DISPID_SpeechDataKey = 2;
pub const DISPID_SDKGetStringValue: DISPID_SpeechDataKey = 4;
pub const DISPID_SDKGetlongValue: DISPID_SpeechDataKey = 6;
pub const DISPID_SDKOpenKey: DISPID_SpeechDataKey = 7;
pub const DISPID_SDKSetBinaryValue: DISPID_SpeechDataKey = 1;
pub const DISPID_SDKSetLongValue: DISPID_SpeechDataKey = 5;
pub const DISPID_SDKSetStringValue: DISPID_SpeechDataKey = 3;
pub const DISPID_SFSClose: DISPID_SpeechFileStream = 101;
pub const DISPID_SFSOpen: DISPID_SpeechFileStream = 100;
pub const DISPID_SGRAddResource: DISPID_SpeechGrammarRule = 6;
pub const DISPID_SGRAddState: DISPID_SpeechGrammarRule = 7;
pub const DISPID_SGRAttributes: DISPID_SpeechGrammarRule = 1;
pub const DISPID_SGRClear: DISPID_SpeechGrammarRule = 5;
pub const DISPID_SGRId: DISPID_SpeechGrammarRule = 4;
pub const DISPID_SGRInitialState: DISPID_SpeechGrammarRule = 2;
pub const DISPID_SGRName: DISPID_SpeechGrammarRule = 3;
pub const DISPID_SGRSAddRuleTransition: DISPID_SpeechGrammarRuleState = 4;
pub const DISPID_SGRSAddSpecialTransition: DISPID_SpeechGrammarRuleState = 5;
pub const DISPID_SGRSAddWordTransition: DISPID_SpeechGrammarRuleState = 3;
pub const DISPID_SGRSRule: DISPID_SpeechGrammarRuleState = 1;
pub const DISPID_SGRSTNextState: DISPID_SpeechGrammarRuleStateTransition = 8;
pub const DISPID_SGRSTPropertyId: DISPID_SpeechGrammarRuleStateTransition = 6;
pub const DISPID_SGRSTPropertyName: DISPID_SpeechGrammarRuleStateTransition = 5;
pub const DISPID_SGRSTPropertyValue: DISPID_SpeechGrammarRuleStateTransition = 7;
pub const DISPID_SGRSTRule: DISPID_SpeechGrammarRuleStateTransition = 3;
pub const DISPID_SGRSTText: DISPID_SpeechGrammarRuleStateTransition = 2;
pub const DISPID_SGRSTType: DISPID_SpeechGrammarRuleStateTransition = 1;
pub const DISPID_SGRSTWeight: DISPID_SpeechGrammarRuleStateTransition = 4;
pub const DISPID_SGRSTransitions: DISPID_SpeechGrammarRuleState = 2;
pub const DISPID_SGRSTsCount: DISPID_SpeechGrammarRuleStateTransitions = 1;
pub const DISPID_SGRSTsItem: DISPID_SpeechGrammarRuleStateTransitions = 0;
pub const DISPID_SGRSTs_NewEnum: DISPID_SpeechGrammarRuleStateTransitions = -4;
pub const DISPID_SGRsAdd: DISPID_SpeechGrammarRules = 3;
pub const DISPID_SGRsCommit: DISPID_SpeechGrammarRules = 4;
pub const DISPID_SGRsCommitAndSave: DISPID_SpeechGrammarRules = 5;
pub const DISPID_SGRsCount: DISPID_SpeechGrammarRules = 1;
pub const DISPID_SGRsDynamic: DISPID_SpeechGrammarRules = 2;
pub const DISPID_SGRsFindRule: DISPID_SpeechGrammarRules = 6;
pub const DISPID_SGRsItem: DISPID_SpeechGrammarRules = 0;
pub const DISPID_SGRs_NewEnum: DISPID_SpeechGrammarRules = -4;
pub const DISPID_SLAddPronunciation: DISPID_SpeechLexicon = 3;
pub const DISPID_SLAddPronunciationByPhoneIds: DISPID_SpeechLexicon = 4;
pub const DISPID_SLGenerationId: DISPID_SpeechLexicon = 1;
pub const DISPID_SLGetGenerationChange: DISPID_SpeechLexicon = 8;
pub const DISPID_SLGetPronunciations: DISPID_SpeechLexicon = 7;
pub const DISPID_SLGetWords: DISPID_SpeechLexicon = 2;
pub const DISPID_SLPLangId: DISPID_SpeechLexiconPronunciation = 2;
pub const DISPID_SLPPartOfSpeech: DISPID_SpeechLexiconPronunciation = 3;
pub const DISPID_SLPPhoneIds: DISPID_SpeechLexiconPronunciation = 4;
pub const DISPID_SLPSymbolic: DISPID_SpeechLexiconPronunciation = 5;
pub const DISPID_SLPType: DISPID_SpeechLexiconPronunciation = 1;
pub const DISPID_SLPsCount: DISPID_SpeechLexiconProns = 1;
pub const DISPID_SLPsItem: DISPID_SpeechLexiconProns = 0;
pub const DISPID_SLPs_NewEnum: DISPID_SpeechLexiconProns = -4;
pub const DISPID_SLRemovePronunciation: DISPID_SpeechLexicon = 5;
pub const DISPID_SLRemovePronunciationByPhoneIds: DISPID_SpeechLexicon = 6;
pub const DISPID_SLWLangId: DISPID_SpeechLexiconWord = 1;
pub const DISPID_SLWPronunciations: DISPID_SpeechLexiconWord = 4;
pub const DISPID_SLWType: DISPID_SpeechLexiconWord = 2;
pub const DISPID_SLWWord: DISPID_SpeechLexiconWord = 3;
pub const DISPID_SLWsCount: DISPID_SpeechLexiconWords = 1;
pub const DISPID_SLWsItem: DISPID_SpeechLexiconWords = 0;
pub const DISPID_SLWs_NewEnum: DISPID_SpeechLexiconWords = -4;
pub const DISPID_SMSADeviceId: DISPID_SpeechMMSysAudio = 300;
pub const DISPID_SMSALineId: DISPID_SpeechMMSysAudio = 301;
pub const DISPID_SMSAMMHandle: DISPID_SpeechMMSysAudio = 302;
pub const DISPID_SMSGetData: DISPID_SpeechMemoryStream = 101;
pub const DISPID_SMSSetData: DISPID_SpeechMemoryStream = 100;
pub const DISPID_SOTCDefault: DISPID_SpeechObjectTokenCategory = 2;
pub const DISPID_SOTCEnumerateTokens: DISPID_SpeechObjectTokenCategory = 5;
pub const DISPID_SOTCGetDataKey: DISPID_SpeechObjectTokenCategory = 4;
pub const DISPID_SOTCId: DISPID_SpeechObjectTokenCategory = 1;
pub const DISPID_SOTCSetId: DISPID_SpeechObjectTokenCategory = 3;
pub const DISPID_SOTCategory: DISPID_SpeechObjectToken = 3;
pub const DISPID_SOTCreateInstance: DISPID_SpeechObjectToken = 7;
pub const DISPID_SOTDataKey: DISPID_SpeechObjectToken = 2;
pub const DISPID_SOTDisplayUI: DISPID_SpeechObjectToken = 12;
pub const DISPID_SOTGetAttribute: DISPID_SpeechObjectToken = 6;
pub const DISPID_SOTGetDescription: DISPID_SpeechObjectToken = 4;
pub const DISPID_SOTGetStorageFileName: DISPID_SpeechObjectToken = 9;
pub const DISPID_SOTId: DISPID_SpeechObjectToken = 1;
pub const DISPID_SOTIsUISupported: DISPID_SpeechObjectToken = 11;
pub const DISPID_SOTMatchesAttributes: DISPID_SpeechObjectToken = 13;
pub const DISPID_SOTRemove: DISPID_SpeechObjectToken = 8;
pub const DISPID_SOTRemoveStorageFileName: DISPID_SpeechObjectToken = 10;
pub const DISPID_SOTSetId: DISPID_SpeechObjectToken = 5;
pub const DISPID_SOTsCount: DISPID_SpeechObjectTokens = 1;
pub const DISPID_SOTsItem: DISPID_SpeechObjectTokens = 0;
pub const DISPID_SOTs_NewEnum: DISPID_SpeechObjectTokens = -4;
pub const DISPID_SPACommit: DISPID_SpeechPhraseAlternate = 5;
pub const DISPID_SPANumberOfElementsInResult: DISPID_SpeechPhraseAlternate = 3;
pub const DISPID_SPAPhraseInfo: DISPID_SpeechPhraseAlternate = 4;
pub const DISPID_SPARecoResult: DISPID_SpeechPhraseAlternate = 1;
pub const DISPID_SPAStartElementInResult: DISPID_SpeechPhraseAlternate = 2;
pub const DISPID_SPAsCount: DISPID_SpeechPhraseAlternates = 1;
pub const DISPID_SPAsItem: DISPID_SpeechPhraseAlternates = 0;
pub const DISPID_SPAs_NewEnum: DISPID_SpeechPhraseAlternates = -4;
pub const DISPID_SPCIdToPhone: DISPID_SpeechPhoneConverter = 3;
pub const DISPID_SPCLangId: DISPID_SpeechPhoneConverter = 1;
pub const DISPID_SPCPhoneToId: DISPID_SpeechPhoneConverter = 2;
pub const DISPID_SPEActualConfidence: DISPID_SpeechPhraseElement = 12;
pub const DISPID_SPEAudioSizeBytes: DISPID_SpeechPhraseElement = 4;
pub const DISPID_SPEAudioSizeTime: DISPID_SpeechPhraseElement = 2;
pub const DISPID_SPEAudioStreamOffset: DISPID_SpeechPhraseElement = 3;
pub const DISPID_SPEAudioTimeOffset: DISPID_SpeechPhraseElement = 1;
pub const DISPID_SPEDisplayAttributes: DISPID_SpeechPhraseElement = 10;
pub const DISPID_SPEDisplayText: DISPID_SpeechPhraseElement = 7;
pub const DISPID_SPEEngineConfidence: DISPID_SpeechPhraseElement = 13;
pub const DISPID_SPELexicalForm: DISPID_SpeechPhraseElement = 8;
pub const DISPID_SPEPronunciation: DISPID_SpeechPhraseElement = 9;
pub const DISPID_SPERequiredConfidence: DISPID_SpeechPhraseElement = 11;
pub const DISPID_SPERetainedSizeBytes: DISPID_SpeechPhraseElement = 6;
pub const DISPID_SPERetainedStreamOffset: DISPID_SpeechPhraseElement = 5;
pub const DISPID_SPEsCount: DISPID_SpeechPhraseElements = 1;
pub const DISPID_SPEsItem: DISPID_SpeechPhraseElements = 0;
pub const DISPID_SPEs_NewEnum: DISPID_SpeechPhraseElements = -4;
pub const DISPID_SPIAudioSizeBytes: DISPID_SpeechPhraseInfo = 5;
pub const DISPID_SPIAudioSizeTime: DISPID_SpeechPhraseInfo = 7;
pub const DISPID_SPIAudioStreamPosition: DISPID_SpeechPhraseInfo = 4;
pub const DISPID_SPIElements: DISPID_SpeechPhraseInfo = 10;
pub const DISPID_SPIEngineId: DISPID_SpeechPhraseInfo = 12;
pub const DISPID_SPIEnginePrivateData: DISPID_SpeechPhraseInfo = 13;
pub const DISPID_SPIGetDisplayAttributes: DISPID_SpeechPhraseInfo = 16;
pub const DISPID_SPIGetText: DISPID_SpeechPhraseInfo = 15;
pub const DISPID_SPIGrammarId: DISPID_SpeechPhraseInfo = 2;
pub const DISPID_SPILanguageId: DISPID_SpeechPhraseInfo = 1;
pub const DISPID_SPIProperties: DISPID_SpeechPhraseInfo = 9;
pub const DISPID_SPIReplacements: DISPID_SpeechPhraseInfo = 11;
pub const DISPID_SPIRetainedSizeBytes: DISPID_SpeechPhraseInfo = 6;
pub const DISPID_SPIRule: DISPID_SpeechPhraseInfo = 8;
pub const DISPID_SPISaveToMemory: DISPID_SpeechPhraseInfo = 14;
pub const DISPID_SPIStartTime: DISPID_SpeechPhraseInfo = 3;
pub const DISPID_SPPBRestorePhraseFromMemory: DISPID_SpeechPhraseBuilder = 1;
pub const DISPID_SPPChildren: DISPID_SpeechPhraseProperty = 9;
pub const DISPID_SPPConfidence: DISPID_SpeechPhraseProperty = 7;
pub const DISPID_SPPEngineConfidence: DISPID_SpeechPhraseProperty = 6;
pub const DISPID_SPPFirstElement: DISPID_SpeechPhraseProperty = 4;
pub const DISPID_SPPId: DISPID_SpeechPhraseProperty = 2;
pub const DISPID_SPPName: DISPID_SpeechPhraseProperty = 1;
pub const DISPID_SPPNumberOfElements: DISPID_SpeechPhraseProperty = 5;
pub const DISPID_SPPParent: DISPID_SpeechPhraseProperty = 8;
pub const DISPID_SPPValue: DISPID_SpeechPhraseProperty = 3;
pub const DISPID_SPPsCount: DISPID_SpeechPhraseProperties = 1;
pub const DISPID_SPPsItem: DISPID_SpeechPhraseProperties = 0;
pub const DISPID_SPPs_NewEnum: DISPID_SpeechPhraseProperties = -4;
pub const DISPID_SPRDisplayAttributes: DISPID_SpeechPhraseReplacement = 1;
pub const DISPID_SPRFirstElement: DISPID_SpeechPhraseReplacement = 3;
pub const DISPID_SPRNumberOfElements: DISPID_SpeechPhraseReplacement = 4;
pub const DISPID_SPRText: DISPID_SpeechPhraseReplacement = 2;
pub const DISPID_SPRsCount: DISPID_SpeechPhraseReplacements = 1;
pub const DISPID_SPRsItem: DISPID_SpeechPhraseReplacements = 0;
pub const DISPID_SPRs_NewEnum: DISPID_SpeechPhraseReplacements = -4;
pub const DISPID_SPRuleChildren: DISPID_SpeechPhraseRule = 6;
pub const DISPID_SPRuleConfidence: DISPID_SpeechPhraseRule = 7;
pub const DISPID_SPRuleEngineConfidence: DISPID_SpeechPhraseRule = 8;
pub const DISPID_SPRuleFirstElement: DISPID_SpeechPhraseRule = 3;
pub const DISPID_SPRuleId: DISPID_SpeechPhraseRule = 2;
pub const DISPID_SPRuleName: DISPID_SpeechPhraseRule = 1;
pub const DISPID_SPRuleNumberOfElements: DISPID_SpeechPhraseRule = 4;
pub const DISPID_SPRuleParent: DISPID_SpeechPhraseRule = 5;
pub const DISPID_SPRulesCount: DISPID_SpeechPhraseRules = 1;
pub const DISPID_SPRulesItem: DISPID_SpeechPhraseRules = 0;
pub const DISPID_SPRules_NewEnum: DISPID_SpeechPhraseRules = -4;
pub const DISPID_SRAllowAudioInputFormatChangesOnNextSet: DISPID_SpeechRecognizer = 2;
pub const DISPID_SRAllowVoiceFormatMatchingOnNextSet: DISPID_SpeechRecoContext = 5;
pub const DISPID_SRAudioInput: DISPID_SpeechRecognizer = 3;
pub const DISPID_SRAudioInputStream: DISPID_SpeechRecognizer = 4;
pub const DISPID_SRCAudioInInterferenceStatus: DISPID_SpeechRecoContext = 2;
pub const DISPID_SRCBookmark: DISPID_SpeechRecoContext = 16;
pub const DISPID_SRCCmdMaxAlternates: DISPID_SpeechRecoContext = 8;
pub const DISPID_SRCCreateGrammar: DISPID_SpeechRecoContext = 14;
pub const DISPID_SRCCreateResultFromMemory: DISPID_SpeechRecoContext = 15;
pub const DISPID_SRCEAdaptation: DISPID_SpeechRecoContextEvents = 15;
pub const DISPID_SRCEAudioLevel: DISPID_SpeechRecoContextEvents = 17;
pub const DISPID_SRCEBookmark: DISPID_SpeechRecoContextEvents = 3;
pub const DISPID_SRCEEndStream: DISPID_SpeechRecoContextEvents = 2;
pub const DISPID_SRCEEnginePrivate: DISPID_SpeechRecoContextEvents = 18;
pub const DISPID_SRCEFalseRecognition: DISPID_SpeechRecoContextEvents = 11;
pub const DISPID_SRCEHypothesis: DISPID_SpeechRecoContextEvents = 8;
pub const DISPID_SRCEInterference: DISPID_SpeechRecoContextEvents = 12;
pub const DISPID_SRCEPhraseStart: DISPID_SpeechRecoContextEvents = 6;
pub const DISPID_SRCEPropertyNumberChange: DISPID_SpeechRecoContextEvents = 9;
pub const DISPID_SRCEPropertyStringChange: DISPID_SpeechRecoContextEvents = 10;
pub const DISPID_SRCERecognition: DISPID_SpeechRecoContextEvents = 7;
pub const DISPID_SRCERecognitionForOtherContext: DISPID_SpeechRecoContextEvents = 16;
pub const DISPID_SRCERecognizerStateChange: DISPID_SpeechRecoContextEvents = 14;
pub const DISPID_SRCERequestUI: DISPID_SpeechRecoContextEvents = 13;
pub const DISPID_SRCESoundEnd: DISPID_SpeechRecoContextEvents = 5;
pub const DISPID_SRCESoundStart: DISPID_SpeechRecoContextEvents = 4;
pub const DISPID_SRCEStartStream: DISPID_SpeechRecoContextEvents = 1;
pub const DISPID_SRCEventInterests: DISPID_SpeechRecoContext = 7;
pub const DISPID_SRCPause: DISPID_SpeechRecoContext = 12;
pub const DISPID_SRCRecognizer: DISPID_SpeechRecoContext = 1;
pub const DISPID_SRCRequestedUIType: DISPID_SpeechRecoContext = 3;
pub const DISPID_SRCResume: DISPID_SpeechRecoContext = 13;
pub const DISPID_SRCRetainedAudio: DISPID_SpeechRecoContext = 10;
pub const DISPID_SRCRetainedAudioFormat: DISPID_SpeechRecoContext = 11;
pub const DISPID_SRCSetAdaptationData: DISPID_SpeechRecoContext = 17;
pub const DISPID_SRCState: DISPID_SpeechRecoContext = 9;
pub const DISPID_SRCVoice: DISPID_SpeechRecoContext = 4;
pub const DISPID_SRCVoicePurgeEvent: DISPID_SpeechRecoContext = 6;
pub const DISPID_SRCreateRecoContext: DISPID_SpeechRecognizer = 10;
pub const DISPID_SRDisplayUI: DISPID_SpeechRecognizer = 17;
pub const DISPID_SREmulateRecognition: DISPID_SpeechRecognizer = 9;
pub const DISPID_SRGCmdLoadFromFile: DISPIDSPRG = 7;
pub const DISPID_SRGCmdLoadFromMemory: DISPIDSPRG = 10;
pub const DISPID_SRGCmdLoadFromObject: DISPIDSPRG = 8;
pub const DISPID_SRGCmdLoadFromProprietaryGrammar: DISPIDSPRG = 11;
pub const DISPID_SRGCmdLoadFromResource: DISPIDSPRG = 9;
pub const DISPID_SRGCmdSetRuleIdState: DISPIDSPRG = 13;
pub const DISPID_SRGCmdSetRuleState: DISPIDSPRG = 12;
pub const DISPID_SRGCommit: DISPIDSPRG = 6;
pub const DISPID_SRGDictationLoad: DISPIDSPRG = 14;
pub const DISPID_SRGDictationSetState: DISPIDSPRG = 16;
pub const DISPID_SRGDictationUnload: DISPIDSPRG = 15;
pub const DISPID_SRGId: DISPIDSPRG = 1;
pub const DISPID_SRGIsPronounceable: DISPIDSPRG = 19;
pub const DISPID_SRGRecoContext: DISPIDSPRG = 2;
pub const DISPID_SRGReset: DISPIDSPRG = 5;
pub const DISPID_SRGRules: DISPIDSPRG = 4;
pub const DISPID_SRGSetTextSelection: DISPIDSPRG = 18;
pub const DISPID_SRGSetWordSequenceData: DISPIDSPRG = 17;
pub const DISPID_SRGState: DISPIDSPRG = 3;
pub const DISPID_SRGetFormat: DISPID_SpeechRecognizer = 11;
pub const DISPID_SRGetPropertyNumber: DISPID_SpeechRecognizer = 13;
pub const DISPID_SRGetPropertyString: DISPID_SpeechRecognizer = 15;
pub const DISPID_SRGetRecognizers: DISPID_SpeechRecognizer = 18;
pub const DISPID_SRIsShared: DISPID_SpeechRecognizer = 5;
pub const DISPID_SRIsUISupported: DISPID_SpeechRecognizer = 16;
pub const DISPID_SRProfile: DISPID_SpeechRecognizer = 8;
pub const DISPID_SRRAlternates: DISPID_SpeechRecoResult = 5;
pub const DISPID_SRRAudio: DISPID_SpeechRecoResult = 6;
pub const DISPID_SRRAudioFormat: DISPID_SpeechRecoResult = 3;
pub const DISPID_SRRDiscardResultInfo: DISPID_SpeechRecoResult = 9;
pub const DISPID_SRRGetXMLErrorInfo: DISPID_SpeechXMLRecoResult = 11;
pub const DISPID_SRRGetXMLResult: DISPID_SpeechXMLRecoResult = 10;
pub const DISPID_SRRPhraseInfo: DISPID_SpeechRecoResult = 4;
pub const DISPID_SRRRecoContext: DISPID_SpeechRecoResult = 1;
pub const DISPID_SRRSaveToMemory: DISPID_SpeechRecoResult = 8;
pub const DISPID_SRRSetTextFeedback: DISPID_SpeechRecoResult2 = 12;
pub const DISPID_SRRSpeakAudio: DISPID_SpeechRecoResult = 7;
pub const DISPID_SRRTLength: DISPID_SpeechRecoResultTimes = 2;
pub const DISPID_SRRTOffsetFromStart: DISPID_SpeechRecoResultTimes = 4;
pub const DISPID_SRRTStreamTime: DISPID_SpeechRecoResultTimes = 1;
pub const DISPID_SRRTTickCount: DISPID_SpeechRecoResultTimes = 3;
pub const DISPID_SRRTimes: DISPID_SpeechRecoResult = 2;
pub const DISPID_SRRecognizer: DISPID_SpeechRecognizer = 1;
pub const DISPID_SRSAudioStatus: DISPID_SpeechRecognizerStatus = 1;
pub const DISPID_SRSClsidEngine: DISPID_SpeechRecognizerStatus = 5;
pub const DISPID_SRSCurrentStreamNumber: DISPID_SpeechRecognizerStatus = 3;
pub const DISPID_SRSCurrentStreamPosition: DISPID_SpeechRecognizerStatus = 2;
pub const DISPID_SRSNumberOfActiveRules: DISPID_SpeechRecognizerStatus = 4;
pub const DISPID_SRSSupportedLanguages: DISPID_SpeechRecognizerStatus = 6;
pub const DISPID_SRSetPropertyNumber: DISPID_SpeechRecognizer = 12;
pub const DISPID_SRSetPropertyString: DISPID_SpeechRecognizer = 14;
pub const DISPID_SRState: DISPID_SpeechRecognizer = 6;
pub const DISPID_SRStatus: DISPID_SpeechRecognizer = 7;
pub const DISPID_SVAlertBoundary: DISPID_SpeechVoice = 10;
pub const DISPID_SVAllowAudioOuputFormatChangesOnNextSet: DISPID_SpeechVoice = 7;
pub const DISPID_SVAudioOutput: DISPID_SpeechVoice = 3;
pub const DISPID_SVAudioOutputStream: DISPID_SpeechVoice = 4;
pub const DISPID_SVDisplayUI: DISPID_SpeechVoice = 22;
pub const DISPID_SVEAudioLevel: DISPID_SpeechVoiceEvent = 9;
pub const DISPID_SVEBookmark: DISPID_SpeechVoiceEvent = 4;
pub const DISPID_SVEEnginePrivate: DISPID_SpeechVoiceEvent = 10;
pub const DISPID_SVEPhoneme: DISPID_SpeechVoiceEvent = 6;
pub const DISPID_SVESentenceBoundary: DISPID_SpeechVoiceEvent = 7;
pub const DISPID_SVEStreamEnd: DISPID_SpeechVoiceEvent = 2;
pub const DISPID_SVEStreamStart: DISPID_SpeechVoiceEvent = 1;
pub const DISPID_SVEViseme: DISPID_SpeechVoiceEvent = 8;
pub const DISPID_SVEVoiceChange: DISPID_SpeechVoiceEvent = 3;
pub const DISPID_SVEWord: DISPID_SpeechVoiceEvent = 5;
pub const DISPID_SVEventInterests: DISPID_SpeechVoice = 8;
pub const DISPID_SVGetAudioInputs: DISPID_SpeechRecognizer = 19;
pub const DISPID_SVGetAudioOutputs: DISPID_SpeechVoice = 18;
pub const DISPID_SVGetProfiles: DISPID_SpeechRecognizer = 20;
pub const DISPID_SVGetVoices: DISPID_SpeechVoice = 17;
pub const DISPID_SVIsUISupported: DISPID_SpeechVoice = 21;
pub const DISPID_SVPause: DISPID_SpeechVoice = 14;
pub const DISPID_SVPriority: DISPID_SpeechVoice = 9;
pub const DISPID_SVRate: DISPID_SpeechVoice = 5;
pub const DISPID_SVResume: DISPID_SpeechVoice = 15;
pub const DISPID_SVSCurrentStreamNumber: DISPID_SpeechVoiceStatus = 1;
pub const DISPID_SVSInputSentenceLength: DISPID_SpeechVoiceStatus = 8;
pub const DISPID_SVSInputSentencePosition: DISPID_SpeechVoiceStatus = 7;
pub const DISPID_SVSInputWordLength: DISPID_SpeechVoiceStatus = 6;
pub const DISPID_SVSInputWordPosition: DISPID_SpeechVoiceStatus = 5;
pub const DISPID_SVSLastBookmark: DISPID_SpeechVoiceStatus = 9;
pub const DISPID_SVSLastBookmarkId: DISPID_SpeechVoiceStatus = 10;
pub const DISPID_SVSLastResult: DISPID_SpeechVoiceStatus = 3;
pub const DISPID_SVSLastStreamNumberQueued: DISPID_SpeechVoiceStatus = 2;
pub const DISPID_SVSPhonemeId: DISPID_SpeechVoiceStatus = 11;
pub const DISPID_SVSRunningState: DISPID_SpeechVoiceStatus = 4;
pub const DISPID_SVSVisemeId: DISPID_SpeechVoiceStatus = 12;
pub const DISPID_SVSkip: DISPID_SpeechVoice = 16;
pub const DISPID_SVSpeak: DISPID_SpeechVoice = 12;
pub const DISPID_SVSpeakCompleteEvent: DISPID_SpeechVoice = 20;
pub const DISPID_SVSpeakStream: DISPID_SpeechVoice = 13;
pub const DISPID_SVStatus: DISPID_SpeechVoice = 1;
pub const DISPID_SVSyncronousSpeakTimeout: DISPID_SpeechVoice = 11;
pub const DISPID_SVVoice: DISPID_SpeechVoice = 2;
pub const DISPID_SVVolume: DISPID_SpeechVoice = 6;
pub const DISPID_SVWaitUntilDone: DISPID_SpeechVoice = 19;
pub const DISPID_SWFEAvgBytesPerSec: DISPID_SpeechWaveFormatEx = 4;
pub const DISPID_SWFEBitsPerSample: DISPID_SpeechWaveFormatEx = 6;
pub const DISPID_SWFEBlockAlign: DISPID_SpeechWaveFormatEx = 5;
pub const DISPID_SWFEChannels: DISPID_SpeechWaveFormatEx = 2;
pub const DISPID_SWFEExtraData: DISPID_SpeechWaveFormatEx = 7;
pub const DISPID_SWFEFormatTag: DISPID_SpeechWaveFormatEx = 1;
pub const DISPID_SWFESamplesPerSec: DISPID_SpeechWaveFormatEx = 3;
pub type DISPID_SpeechAudio = i32;
pub type DISPID_SpeechAudioBufferInfo = i32;
pub type DISPID_SpeechAudioFormat = i32;
pub type DISPID_SpeechAudioStatus = i32;
pub type DISPID_SpeechBaseStream = i32;
pub type DISPID_SpeechCustomStream = i32;
pub type DISPID_SpeechDataKey = i32;
pub type DISPID_SpeechFileStream = i32;
pub type DISPID_SpeechGrammarRule = i32;
pub type DISPID_SpeechGrammarRuleState = i32;
pub type DISPID_SpeechGrammarRuleStateTransition = i32;
pub type DISPID_SpeechGrammarRuleStateTransitions = i32;
pub type DISPID_SpeechGrammarRules = i32;
pub type DISPID_SpeechLexicon = i32;
pub type DISPID_SpeechLexiconProns = i32;
pub type DISPID_SpeechLexiconPronunciation = i32;
pub type DISPID_SpeechLexiconWord = i32;
pub type DISPID_SpeechLexiconWords = i32;
pub type DISPID_SpeechMMSysAudio = i32;
pub type DISPID_SpeechMemoryStream = i32;
pub type DISPID_SpeechObjectToken = i32;
pub type DISPID_SpeechObjectTokenCategory = i32;
pub type DISPID_SpeechObjectTokens = i32;
pub type DISPID_SpeechPhoneConverter = i32;
pub type DISPID_SpeechPhraseAlternate = i32;
pub type DISPID_SpeechPhraseAlternates = i32;
pub type DISPID_SpeechPhraseBuilder = i32;
pub type DISPID_SpeechPhraseElement = i32;
pub type DISPID_SpeechPhraseElements = i32;
pub type DISPID_SpeechPhraseInfo = i32;
pub type DISPID_SpeechPhraseProperties = i32;
pub type DISPID_SpeechPhraseProperty = i32;
pub type DISPID_SpeechPhraseReplacement = i32;
pub type DISPID_SpeechPhraseReplacements = i32;
pub type DISPID_SpeechPhraseRule = i32;
pub type DISPID_SpeechPhraseRules = i32;
pub type DISPID_SpeechRecoContext = i32;
pub type DISPID_SpeechRecoContextEvents = i32;
pub type DISPID_SpeechRecoResult = i32;
pub type DISPID_SpeechRecoResult2 = i32;
pub type DISPID_SpeechRecoResultTimes = i32;
pub type DISPID_SpeechRecognizer = i32;
pub type DISPID_SpeechRecognizerStatus = i32;
pub type DISPID_SpeechVoice = i32;
pub type DISPID_SpeechVoiceEvent = i32;
pub type DISPID_SpeechVoiceStatus = i32;
pub type DISPID_SpeechWaveFormatEx = i32;
pub type DISPID_SpeechXMLRecoResult = i32;
pub const OrderedSubset: SPMATCHINGMODE = 3;
pub const OrderedSubsetContentRequired: SPMATCHINGMODE = 7;
pub const PA_Ipa: PHONETICALPHABET = 0;
pub const PA_Sapi: PHONETICALPHABET = 2;
pub const PA_Ups: PHONETICALPHABET = 1;
pub type PCSPPHONEID = windows_sys::core::PCWSTR;
pub type PHONETICALPHABET = i32;
pub type PSPPHONEID = windows_sys::core::PWSTR;
pub const SAFT11kHz16BitMono: SpeechAudioFormatType = 10;
pub const SAFT11kHz16BitStereo: SpeechAudioFormatType = 11;
pub const SAFT11kHz8BitMono: SpeechAudioFormatType = 8;
pub const SAFT11kHz8BitStereo: SpeechAudioFormatType = 9;
pub const SAFT12kHz16BitMono: SpeechAudioFormatType = 14;
pub const SAFT12kHz16BitStereo: SpeechAudioFormatType = 15;
pub const SAFT12kHz8BitMono: SpeechAudioFormatType = 12;
pub const SAFT12kHz8BitStereo: SpeechAudioFormatType = 13;
pub const SAFT16kHz16BitMono: SpeechAudioFormatType = 18;
pub const SAFT16kHz16BitStereo: SpeechAudioFormatType = 19;
pub const SAFT16kHz8BitMono: SpeechAudioFormatType = 16;
pub const SAFT16kHz8BitStereo: SpeechAudioFormatType = 17;
pub const SAFT22kHz16BitMono: SpeechAudioFormatType = 22;
pub const SAFT22kHz16BitStereo: SpeechAudioFormatType = 23;
pub const SAFT22kHz8BitMono: SpeechAudioFormatType = 20;
pub const SAFT22kHz8BitStereo: SpeechAudioFormatType = 21;
pub const SAFT24kHz16BitMono: SpeechAudioFormatType = 26;
pub const SAFT24kHz16BitStereo: SpeechAudioFormatType = 27;
pub const SAFT24kHz8BitMono: SpeechAudioFormatType = 24;
pub const SAFT24kHz8BitStereo: SpeechAudioFormatType = 25;
pub const SAFT32kHz16BitMono: SpeechAudioFormatType = 30;
pub const SAFT32kHz16BitStereo: SpeechAudioFormatType = 31;
pub const SAFT32kHz8BitMono: SpeechAudioFormatType = 28;
pub const SAFT32kHz8BitStereo: SpeechAudioFormatType = 29;
pub const SAFT44kHz16BitMono: SpeechAudioFormatType = 34;
pub const SAFT44kHz16BitStereo: SpeechAudioFormatType = 35;
pub const SAFT44kHz8BitMono: SpeechAudioFormatType = 32;
pub const SAFT44kHz8BitStereo: SpeechAudioFormatType = 33;
pub const SAFT48kHz16BitMono: SpeechAudioFormatType = 38;
pub const SAFT48kHz16BitStereo: SpeechAudioFormatType = 39;
pub const SAFT48kHz8BitMono: SpeechAudioFormatType = 36;
pub const SAFT48kHz8BitStereo: SpeechAudioFormatType = 37;
pub const SAFT8kHz16BitMono: SpeechAudioFormatType = 6;
pub const SAFT8kHz16BitStereo: SpeechAudioFormatType = 7;
pub const SAFT8kHz8BitMono: SpeechAudioFormatType = 4;
pub const SAFT8kHz8BitStereo: SpeechAudioFormatType = 5;
pub const SAFTADPCM_11kHzMono: SpeechAudioFormatType = 59;
pub const SAFTADPCM_11kHzStereo: SpeechAudioFormatType = 60;
pub const SAFTADPCM_22kHzMono: SpeechAudioFormatType = 61;
pub const SAFTADPCM_22kHzStereo: SpeechAudioFormatType = 62;
pub const SAFTADPCM_44kHzMono: SpeechAudioFormatType = 63;
pub const SAFTADPCM_44kHzStereo: SpeechAudioFormatType = 64;
pub const SAFTADPCM_8kHzMono: SpeechAudioFormatType = 57;
pub const SAFTADPCM_8kHzStereo: SpeechAudioFormatType = 58;
pub const SAFTCCITT_ALaw_11kHzMono: SpeechAudioFormatType = 43;
pub const SAFTCCITT_ALaw_11kHzStereo: SpeechAudioFormatType = 44;
pub const SAFTCCITT_ALaw_22kHzMono: SpeechAudioFormatType = 45;
pub const SAFTCCITT_ALaw_22kHzStereo: SpeechAudioFormatType = 46;
pub const SAFTCCITT_ALaw_44kHzMono: SpeechAudioFormatType = 47;
pub const SAFTCCITT_ALaw_44kHzStereo: SpeechAudioFormatType = 48;
pub const SAFTCCITT_ALaw_8kHzMono: SpeechAudioFormatType = 41;
pub const SAFTCCITT_ALaw_8kHzStereo: SpeechAudioFormatType = 42;
pub const SAFTCCITT_uLaw_11kHzMono: SpeechAudioFormatType = 51;
pub const SAFTCCITT_uLaw_11kHzStereo: SpeechAudioFormatType = 52;
pub const SAFTCCITT_uLaw_22kHzMono: SpeechAudioFormatType = 53;
pub const SAFTCCITT_uLaw_22kHzStereo: SpeechAudioFormatType = 54;
pub const SAFTCCITT_uLaw_44kHzMono: SpeechAudioFormatType = 55;
pub const SAFTCCITT_uLaw_44kHzStereo: SpeechAudioFormatType = 56;
pub const SAFTCCITT_uLaw_8kHzMono: SpeechAudioFormatType = 49;
pub const SAFTCCITT_uLaw_8kHzStereo: SpeechAudioFormatType = 50;
pub const SAFTDefault: SpeechAudioFormatType = -1;
pub const SAFTExtendedAudioFormat: SpeechAudioFormatType = 3;
pub const SAFTGSM610_11kHzMono: SpeechAudioFormatType = 66;
pub const SAFTGSM610_22kHzMono: SpeechAudioFormatType = 67;
pub const SAFTGSM610_44kHzMono: SpeechAudioFormatType = 68;
pub const SAFTGSM610_8kHzMono: SpeechAudioFormatType = 65;
pub const SAFTNoAssignedFormat: SpeechAudioFormatType = 0;
pub const SAFTNonStandardFormat: SpeechAudioFormatType = 2;
pub const SAFTText: SpeechAudioFormatType = 1;
pub const SAFTTrueSpeech_8kHz1BitMono: SpeechAudioFormatType = 40;
pub const SASClosed: SpeechAudioState = 0;
pub const SASPause: SpeechAudioState = 2;
pub const SASRun: SpeechAudioState = 3;
pub const SASStop: SpeechAudioState = 1;
pub const SBONone: SpeechBookmarkOptions = 0;
pub const SBOPause: SpeechBookmarkOptions = 1;
pub const SDA_Consume_Leading_Spaces: SpeechDisplayAttributes = 8;
pub const SDA_No_Trailing_Space: SpeechDisplayAttributes = 0;
pub const SDA_One_Trailing_Space: SpeechDisplayAttributes = 2;
pub const SDA_Two_Trailing_Spaces: SpeechDisplayAttributes = 4;
pub const SDKLCurrentConfig: SpeechDataKeyLocation = 5;
pub const SDKLCurrentUser: SpeechDataKeyLocation = 1;
pub const SDKLDefaultLocation: SpeechDataKeyLocation = 0;
pub const SDKLLocalMachine: SpeechDataKeyLocation = 2;
pub const SDTAll: SpeechDiscardType = 255;
pub const SDTAlternates: SpeechDiscardType = 128;
pub const SDTAudio: SpeechDiscardType = 64;
pub const SDTDisplayText: SpeechDiscardType = 8;
pub const SDTLexicalForm: SpeechDiscardType = 16;
pub const SDTPronunciation: SpeechDiscardType = 32;
pub const SDTProperty: SpeechDiscardType = 1;
pub const SDTReplacement: SpeechDiscardType = 2;
pub const SDTRule: SpeechDiscardType = 4;
pub const SECFDefault: SpeechEmulationCompareFlags = 196609;
pub const SECFEmulateResult: SpeechEmulationCompareFlags = 1073741824;
pub const SECFIgnoreCase: SpeechEmulationCompareFlags = 1;
pub const SECFIgnoreKanaType: SpeechEmulationCompareFlags = 65536;
pub const SECFIgnoreWidth: SpeechEmulationCompareFlags = 131072;
pub const SECFNoSpecialChars: SpeechEmulationCompareFlags = 536870912;
pub const SECHighConfidence: SpeechEngineConfidence = 1;
pub const SECLowConfidence: SpeechEngineConfidence = -1;
pub const SECNormalConfidence: SpeechEngineConfidence = 0;
pub const SFTInput: SpeechFormatType = 0;
pub const SFTSREngine: SpeechFormatType = 1;
pub const SGDSActive: SpeechRuleState = 1;
pub const SGDSActiveUserDelimited: SpeechRuleState = 4;
pub const SGDSActiveWithAutoPause: SpeechRuleState = 3;
pub const SGDSInactive: SpeechRuleState = 0;
pub const SGDisplay: SpeechGrammarWordType = 0;
pub const SGLexical: SpeechGrammarWordType = 1;
pub const SGLexicalNoSpecialChars: SpeechGrammarWordType = 3;
pub const SGPronounciation: SpeechGrammarWordType = 2;
pub const SGRSTTDictation: SpeechGrammarRuleStateTransitionType = 3;
pub const SGRSTTEpsilon: SpeechGrammarRuleStateTransitionType = 0;
pub const SGRSTTRule: SpeechGrammarRuleStateTransitionType = 2;
pub const SGRSTTTextBuffer: SpeechGrammarRuleStateTransitionType = 5;
pub const SGRSTTWildcard: SpeechGrammarRuleStateTransitionType = 4;
pub const SGRSTTWord: SpeechGrammarRuleStateTransitionType = 1;
pub const SGSDisabled: SpeechGrammarState = 0;
pub const SGSEnabled: SpeechGrammarState = 1;
pub const SGSExclusive: SpeechGrammarState = 3;
pub const SINoSignal: SpeechInterference = 2;
pub const SINoise: SpeechInterference = 1;
pub const SINone: SpeechInterference = 0;
pub const SITooFast: SpeechInterference = 5;
pub const SITooLoud: SpeechInterference = 3;
pub const SITooQuiet: SpeechInterference = 4;
pub const SITooSlow: SpeechInterference = 6;
pub const SLODynamic: SpeechLoadOption = 1;
pub const SLOStatic: SpeechLoadOption = 0;
pub const SLTApp: SpeechLexiconType = 2;
pub const SLTUser: SpeechLexiconType = 1;
pub type SPADAPTATIONRELEVANCE = i32;
pub type SPADAPTATIONSETTINGS = i32;
pub const SPADS_CurrentRecognizer: SPADAPTATIONSETTINGS = 1;
pub const SPADS_Default: SPADAPTATIONSETTINGS = 0;
pub const SPADS_HighVolumeDataSource: SPADAPTATIONSETTINGS = 16;
pub const SPADS_Immediate: SPADAPTATIONSETTINGS = 4;
pub const SPADS_RecoProfile: SPADAPTATIONSETTINGS = 2;
pub const SPADS_Reset: SPADAPTATIONSETTINGS = 8;
pub const SPAF_ALL: SPDISPLAYATTRIBUTES = 31;
pub const SPAF_BUFFER_POSITION: SPDISPLAYATTRIBUTES = 16;
pub const SPAF_CONSUME_LEADING_SPACES: SPDISPLAYATTRIBUTES = 8;
pub const SPAF_ONE_TRAILING_SPACE: SPDISPLAYATTRIBUTES = 2;
pub const SPAF_TWO_TRAILING_SPACES: SPDISPLAYATTRIBUTES = 4;
pub const SPAF_USER_SPECIFIED: SPDISPLAYATTRIBUTES = 128;
pub const SPAO_NONE: SPAUDIOOPTIONS = 0;
pub const SPAO_RETAIN_AUDIO: SPAUDIOOPTIONS = 1;
pub const SPAR_High: SPADAPTATIONRELEVANCE = 3;
pub const SPAR_Low: SPADAPTATIONRELEVANCE = 1;
pub const SPAR_Medium: SPADAPTATIONRELEVANCE = 2;
pub const SPAR_Unknown: SPADAPTATIONRELEVANCE = 0;
pub const SPAS_CLOSED: SPAUDIOSTATE = 0;
pub const SPAS_PAUSE: SPAUDIOSTATE = 2;
pub const SPAS_RUN: SPAUDIOSTATE = 3;
pub const SPAS_STOP: SPAUDIOSTATE = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SPAUDIOBUFFERINFO {
    pub ulMsMinNotification: u32,
    pub ulMsBufferSize: u32,
    pub ulMsEventBias: u32,
}
pub type SPAUDIOOPTIONS = i32;
pub type SPAUDIOSTATE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SPAUDIOSTATUS {
    pub cbFreeBuffSpace: i32,
    pub cbNonBlockingIO: u32,
    pub State: SPAUDIOSTATE,
    pub CurSeekPos: u64,
    pub CurDevicePos: u64,
    pub dwAudioLevel: u32,
    pub dwReserved2: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SPBINARYGRAMMAR {
    pub ulTotalSerializedSize: u32,
}
pub type SPBOOKMARKOPTIONS = i32;
pub const SPBO_AHEAD: SPBOOKMARKOPTIONS = 2;
pub const SPBO_NONE: SPBOOKMARKOPTIONS = 0;
pub const SPBO_PAUSE: SPBOOKMARKOPTIONS = 1;
pub const SPBO_TIME_UNITS: SPBOOKMARKOPTIONS = 4;
pub type SPCATEGORYTYPE = i32;
pub const SPCAT_APPLEXICONS: windows_sys::core::PCWSTR = windows_sys::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AppLexicons");
pub const SPCAT_AUDIOIN: windows_sys::core::PCWSTR = windows_sys::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioInput");
pub const SPCAT_AUDIOOUT: windows_sys::core::PCWSTR = windows_sys::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioOutput");
pub const SPCAT_PHONECONVERTERS: windows_sys::core::PCWSTR = windows_sys::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\PhoneConverters");
pub const SPCAT_RECOGNIZERS: windows_sys::core::PCWSTR = windows_sys::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\Recognizers");
pub const SPCAT_RECOPROFILES: windows_sys::core::PCWSTR = windows_sys::core::w!("HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech\\RecoProfiles");
pub const SPCAT_TEXTNORMALIZERS: windows_sys::core::PCWSTR = windows_sys::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\TextNormalizers");
pub const SPCAT_VOICES: windows_sys::core::PCWSTR = windows_sys::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\Voices");
pub type SPCFGRULEATTRIBUTES = i32;
pub const SPCF_ADD_TO_USER_LEXICON: SPCOMMITFLAGS = 1;
pub const SPCF_DEFINITE_CORRECTION: SPCOMMITFLAGS = 2;
pub const SPCF_NONE: SPCOMMITFLAGS = 0;
pub type SPCOMMITFLAGS = i32;
pub type SPCONTEXTSTATE = i32;
pub const SPCS_DISABLED: SPCONTEXTSTATE = 0;
pub const SPCS_ENABLED: SPCONTEXTSTATE = 1;
pub const SPCT_COMMAND: SPCATEGORYTYPE = 0;
pub const SPCT_DICTATION: SPCATEGORYTYPE = 1;
pub const SPCT_SLEEP: SPCATEGORYTYPE = 2;
pub const SPCT_SUB_COMMAND: SPCATEGORYTYPE = 3;
pub const SPCT_SUB_DICTATION: SPCATEGORYTYPE = 4;
pub const SPCURRENT_USER_LEXICON_TOKEN_ID: windows_sys::core::PCWSTR = windows_sys::core::w!("HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech\\CurrentUserLexicon");
pub const SPCURRENT_USER_SHORTCUT_TOKEN_ID: windows_sys::core::PCWSTR = windows_sys::core::w!("HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech\\CurrentUserShortcut");
pub type SPDATAKEYLOCATION = i32;
pub const SPDF_ALL: SPVALUETYPE = 255;
pub const SPDF_ALTERNATES: SPVALUETYPE = 128;
pub const SPDF_AUDIO: SPVALUETYPE = 64;
pub const SPDF_DISPLAYTEXT: SPVALUETYPE = 8;
pub const SPDF_LEXICALFORM: SPVALUETYPE = 16;
pub const SPDF_PRONUNCIATION: SPVALUETYPE = 32;
pub const SPDF_PROPERTY: SPVALUETYPE = 1;
pub const SPDF_REPLACEMENT: SPVALUETYPE = 2;
pub const SPDF_RULE: SPVALUETYPE = 4;
pub const SPDICTATION: windows_sys::core::PCWSTR = windows_sys::core::w!("*");
pub type SPDISPLAYATTRIBUTES = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPDISPLAYPHRASE {
    pub ulNumTokens: u32,
    pub pTokens: *mut SPDISPLAYTOKEN,
}
impl Default for SPDISPLAYPHRASE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPDISPLAYTOKEN {
    pub pszLexical: *const u16,
    pub pszDisplay: *const u16,
    pub bDisplayAttributes: u8,
}
impl Default for SPDISPLAYTOKEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SPDKL_CurrentConfig: SPDATAKEYLOCATION = 5;
pub const SPDKL_CurrentUser: SPDATAKEYLOCATION = 1;
pub const SPDKL_DefaultLocation: SPDATAKEYLOCATION = 0;
pub const SPDKL_LocalMachine: SPDATAKEYLOCATION = 2;
pub const SPDUI_AddRemoveWord: windows_sys::core::PCWSTR = windows_sys::core::w!("AddRemoveWord");
pub const SPDUI_AudioProperties: windows_sys::core::PCWSTR = windows_sys::core::w!("AudioProperties");
pub const SPDUI_AudioVolume: windows_sys::core::PCWSTR = windows_sys::core::w!("AudioVolume");
pub const SPDUI_EngineProperties: windows_sys::core::PCWSTR = windows_sys::core::w!("EngineProperties");
pub const SPDUI_MicTraining: windows_sys::core::PCWSTR = windows_sys::core::w!("MicTraining");
pub const SPDUI_RecoProfileProperties: windows_sys::core::PCWSTR = windows_sys::core::w!("RecoProfileProperties");
pub const SPDUI_ShareData: windows_sys::core::PCWSTR = windows_sys::core::w!("ShareData");
pub const SPDUI_Tutorial: windows_sys::core::PCWSTR = windows_sys::core::w!("Tutorial");
pub const SPDUI_UserEnrollment: windows_sys::core::PCWSTR = windows_sys::core::w!("UserEnrollment");
pub const SPDUI_UserTraining: windows_sys::core::PCWSTR = windows_sys::core::w!("UserTraining");
pub type SPEAKFLAGS = i32;
pub const SPEI_ACTIVE_CATEGORY_CHANGED: SPEVENTENUM = 53;
pub const SPEI_ADAPTATION: SPEVENTENUM = 47;
pub const SPEI_END_INPUT_STREAM: SPEVENTENUM = 2;
pub const SPEI_END_SR_STREAM: SPEVENTENUM = 34;
pub const SPEI_FALSE_RECOGNITION: SPEVENTENUM = 43;
pub const SPEI_HYPOTHESIS: SPEVENTENUM = 39;
pub const SPEI_INTERFERENCE: SPEVENTENUM = 44;
pub const SPEI_MAX_SR: SPEVENTENUM = 55;
pub const SPEI_MAX_TTS: SPEVENTENUM = 15;
pub const SPEI_MIN_SR: SPEVENTENUM = 34;
pub const SPEI_MIN_TTS: SPEVENTENUM = 1;
pub const SPEI_PHONEME: SPEVENTENUM = 6;
pub const SPEI_PHRASE_START: SPEVENTENUM = 37;
pub const SPEI_PROPERTY_NUM_CHANGE: SPEVENTENUM = 41;
pub const SPEI_PROPERTY_STRING_CHANGE: SPEVENTENUM = 42;
pub const SPEI_RECOGNITION: SPEVENTENUM = 38;
pub const SPEI_RECO_OTHER_CONTEXT: SPEVENTENUM = 49;
pub const SPEI_RECO_STATE_CHANGE: SPEVENTENUM = 46;
pub const SPEI_REQUEST_UI: SPEVENTENUM = 45;
pub const SPEI_RESERVED1: SPEVENTENUM = 30;
pub const SPEI_RESERVED2: SPEVENTENUM = 33;
pub const SPEI_RESERVED3: SPEVENTENUM = 63;
pub const SPEI_RESERVED5: SPEVENTENUM = 54;
pub const SPEI_RESERVED6: SPEVENTENUM = 55;
pub const SPEI_SENTENCE_BOUNDARY: SPEVENTENUM = 7;
pub const SPEI_SOUND_END: SPEVENTENUM = 36;
pub const SPEI_SOUND_START: SPEVENTENUM = 35;
pub const SPEI_SR_AUDIO_LEVEL: SPEVENTENUM = 50;
pub const SPEI_SR_BOOKMARK: SPEVENTENUM = 40;
pub const SPEI_SR_PRIVATE: SPEVENTENUM = 52;
pub const SPEI_SR_RETAINEDAUDIO: SPEVENTENUM = 51;
pub const SPEI_START_INPUT_STREAM: SPEVENTENUM = 1;
pub const SPEI_START_SR_STREAM: SPEVENTENUM = 48;
pub const SPEI_TTS_AUDIO_LEVEL: SPEVENTENUM = 9;
pub const SPEI_TTS_BOOKMARK: SPEVENTENUM = 4;
pub const SPEI_TTS_PRIVATE: SPEVENTENUM = 15;
pub const SPEI_UNDEFINED: SPEVENTENUM = 0;
pub const SPEI_VISEME: SPEVENTENUM = 8;
pub const SPEI_VOICE_CHANGE: SPEVENTENUM = 3;
pub const SPEI_WORD_BOUNDARY: SPEVENTENUM = 5;
pub type SPENDSRSTREAMFLAGS = i32;
pub const SPESF_EMULATED: SPENDSRSTREAMFLAGS = 2;
pub const SPESF_NONE: SPENDSRSTREAMFLAGS = 0;
pub const SPESF_STREAM_RELEASED: SPENDSRSTREAMFLAGS = 1;
pub const SPET_LPARAM_IS_OBJECT: SPEVENTLPARAMTYPE = 2;
pub const SPET_LPARAM_IS_POINTER: SPEVENTLPARAMTYPE = 3;
pub const SPET_LPARAM_IS_STRING: SPEVENTLPARAMTYPE = 4;
pub const SPET_LPARAM_IS_TOKEN: SPEVENTLPARAMTYPE = 1;
pub const SPET_LPARAM_IS_UNDEFINED: SPEVENTLPARAMTYPE = 0;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Default)]
pub struct SPEVENT {
    pub _bitfield: SPEVENTENUM,
    pub ulStreamNum: u32,
    pub ullAudioStreamOffset: u64,
    pub wParam: super::minwindef::WPARAM,
    pub lParam: super::minwindef::LPARAM,
}
pub type SPEVENTENUM = i32;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Default)]
pub struct SPEVENTEX {
    pub _bitfield: SPEVENTENUM,
    pub ulStreamNum: u32,
    pub ullAudioStreamOffset: u64,
    pub wParam: super::minwindef::WPARAM,
    pub lParam: super::minwindef::LPARAM,
    pub ullAudioTimeOffset: u64,
}
pub type SPEVENTLPARAMTYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SPEVENTSOURCEINFO {
    pub ullEventInterest: u64,
    pub ullQueuedInterest: u64,
    pub ulCount: u32,
}
pub const SPFEI_ALL_EVENTS: i32 = -1;
pub const SPFEI_ALL_SR_EVENTS: u32 = 1073741824;
pub const SPFEI_ALL_TTS_EVENTS: u32 = 1073807358;
pub const SPFEI_FLAGCHECK: u32 = 1073741824;
pub type SPFILEMODE = i32;
pub const SPFM_CREATE: SPFILEMODE = 2;
pub const SPFM_CREATE_ALWAYS: SPFILEMODE = 3;
pub const SPFM_NUM_MODES: SPFILEMODE = 4;
pub const SPFM_OPEN_READONLY: SPFILEMODE = 0;
pub const SPFM_OPEN_READWRITE: SPFILEMODE = 1;
pub const SPF_ASYNC: SPEAKFLAGS = 1;
pub const SPF_DEFAULT: SPEAKFLAGS = 0;
pub const SPF_IS_FILENAME: SPEAKFLAGS = 4;
pub const SPF_IS_NOT_XML: SPEAKFLAGS = 16;
pub const SPF_IS_XML: SPEAKFLAGS = 8;
pub const SPF_NLP_MASK: SPEAKFLAGS = 64;
pub const SPF_NLP_SPEAK_PUNC: SPEAKFLAGS = 64;
pub const SPF_PARSE_AUTODETECT: SPEAKFLAGS = 0;
pub const SPF_PARSE_MASK: SPEAKFLAGS = 384;
pub const SPF_PARSE_SAPI: SPEAKFLAGS = 128;
pub const SPF_PARSE_SSML: SPEAKFLAGS = 256;
pub const SPF_PERSIST_XML: SPEAKFLAGS = 32;
pub const SPF_PURGEBEFORESPEAK: SPEAKFLAGS = 2;
pub const SPF_UNUSED_FLAGS: SPEAKFLAGS = -512;
pub const SPF_VOICE_MASK: SPEAKFLAGS = 511;
pub const SPGO_ALL: SPGRAMMAROPTIONS = 1023;
pub const SPGO_DEFAULT: SPGRAMMAROPTIONS = 1019;
pub const SPGO_FILE: SPGRAMMAROPTIONS = 16;
pub const SPGO_HTTP: SPGRAMMAROPTIONS = 32;
pub const SPGO_OBJECT: SPGRAMMAROPTIONS = 128;
pub const SPGO_RES: SPGRAMMAROPTIONS = 64;
pub const SPGO_SAPI: SPGRAMMAROPTIONS = 1;
pub const SPGO_SRGS: SPGRAMMAROPTIONS = 2;
pub const SPGO_SRGS_MS_SCRIPT: SPGRAMMAROPTIONS = 8;
pub const SPGO_SRGS_SCRIPT: SPGRAMMAROPTIONS = 778;
pub const SPGO_SRGS_STG_SCRIPT: SPGRAMMAROPTIONS = 512;
pub const SPGO_SRGS_W3C_SCRIPT: SPGRAMMAROPTIONS = 256;
pub const SPGO_UPS: SPGRAMMAROPTIONS = 4;
pub type SPGRAMMAROPTIONS = i32;
pub type SPGRAMMARSTATE = i32;
pub type SPGRAMMARWORDTYPE = i32;
pub const SPGS_DISABLED: SPGRAMMARSTATE = 0;
pub const SPGS_ENABLED: SPGRAMMARSTATE = 1;
pub const SPGS_EXCLUSIVE: SPGRAMMARSTATE = 3;
pub const SPINFDICTATION: windows_sys::core::PCWSTR = windows_sys::core::w!("*+");
pub type SPINTERFERENCE = i32;
pub const SPINTERFERENCE_LATENCY_TRUNCATE_BEGIN: SPINTERFERENCE = 8;
pub const SPINTERFERENCE_LATENCY_TRUNCATE_END: SPINTERFERENCE = 9;
pub const SPINTERFERENCE_LATENCY_WARNING: SPINTERFERENCE = 7;
pub const SPINTERFERENCE_NOISE: SPINTERFERENCE = 1;
pub const SPINTERFERENCE_NONE: SPINTERFERENCE = 0;
pub const SPINTERFERENCE_NOSIGNAL: SPINTERFERENCE = 2;
pub const SPINTERFERENCE_TOOFAST: SPINTERFERENCE = 5;
pub const SPINTERFERENCE_TOOLOUD: SPINTERFERENCE = 3;
pub const SPINTERFERENCE_TOOQUIET: SPINTERFERENCE = 4;
pub const SPINTERFERENCE_TOOSLOW: SPINTERFERENCE = 6;
pub type SPLEXICONTYPE = i32;
pub type SPLOADOPTIONS = i32;
pub const SPLO_DYNAMIC: SPLOADOPTIONS = 1;
pub const SPLO_STATIC: SPLOADOPTIONS = 0;
pub type SPMATCHINGMODE = i32;
pub const SPMAX_RATE: SPVLIMITS = 10;
pub const SPMAX_VOLUME: SPVLIMITS = 100;
pub const SPMIN_RATE: SPVLIMITS = -10;
pub const SPMIN_VOLUME: SPVLIMITS = 0;
pub const SPMMSYS_AUDIO_IN_TOKEN_ID: windows_sys::core::PCWSTR = windows_sys::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioInput\\TokenEnums\\MMAudioIn\\");
pub const SPMMSYS_AUDIO_OUT_TOKEN_ID: windows_sys::core::PCWSTR = windows_sys::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioOutput\\TokenEnums\\MMAudioOut\\");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPNORMALIZATIONLIST {
    pub ulSize: u32,
    pub ppszzNormalizedList: *mut *mut u16,
}
impl Default for SPNORMALIZATIONLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
pub type SPNOTIFYCALLBACK = Option<unsafe extern "system" fn(wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM)>;
pub type SPPARTOFSPEECH = i32;
pub type SPPHONEID = u16;
#[repr(C)]
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct SPPHRASE {
    pub Base: SPPHRASE_53,
    pub SemanticTagFormat: SPSEMANTICFORMAT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPPHRASEELEMENT {
    pub ulAudioTimeOffset: u32,
    pub ulAudioSizeTime: u32,
    pub ulAudioStreamOffset: u32,
    pub ulAudioSizeBytes: u32,
    pub ulRetainedStreamOffset: u32,
    pub ulRetainedSizeBytes: u32,
    pub pszDisplayText: windows_sys::core::PCWSTR,
    pub pszLexicalForm: windows_sys::core::PCWSTR,
    pub pszPronunciation: *const SPPHONEID,
    pub bDisplayAttributes: u8,
    pub RequiredConfidence: i8,
    pub ActualConfidence: i8,
    pub Reserved: u8,
    pub SREngineConfidence: f32,
}
impl Default for SPPHRASEELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct SPPHRASEPROPERTY {
    pub pszName: windows_sys::core::PCWSTR,
    pub Anonymous: SPPHRASEPROPERTY_0,
    pub pszValue: windows_sys::core::PCWSTR,
    pub vValue: super::oaidl::VARIANT,
    pub ulFirstElement: u32,
    pub ulCountOfElements: u32,
    pub pNextSibling: *const Self,
    pub pFirstChild: *const Self,
    pub SREngineConfidence: f32,
    pub Confidence: i8,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for SPPHRASEPROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub union SPPHRASEPROPERTY_0 {
    pub ulId: u32,
    pub Anonymous: SPPHRASEPROPERTY_0_0,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for SPPHRASEPROPERTY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct SPPHRASEPROPERTY_0_0 {
    pub bType: super::rpcndr::byte,
    pub bReserved: super::rpcndr::byte,
    pub usArrayIndex: u16,
}
pub type SPPHRASEPROPERTYUNIONTYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPPHRASEREPLACEMENT {
    pub bDisplayAttributes: u8,
    pub pszReplacementText: windows_sys::core::PCWSTR,
    pub ulFirstElement: u32,
    pub ulCountOfElements: u32,
}
impl Default for SPPHRASEREPLACEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SPPHRASERNG = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPPHRASERULE {
    pub pszName: windows_sys::core::PCWSTR,
    pub ulId: u32,
    pub ulFirstElement: u32,
    pub ulCountOfElements: u32,
    pub pNextSibling: *const Self,
    pub pFirstChild: *const Self,
    pub SREngineConfidence: f32,
    pub Confidence: i8,
}
impl Default for SPPHRASERULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
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
    pub pProperties: *const SPPHRASEPROPERTY,
    pub pElements: *const SPPHRASEELEMENT,
    pub cReplacements: u32,
    pub pReplacements: *const SPPHRASEREPLACEMENT,
    pub SREngineID: windows_sys::core::GUID,
    pub ulSREnginePrivateDataSize: u32,
    pub pSREnginePrivateData: *const u8,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for SPPHRASE_50 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct SPPHRASE_53 {
    pub Base: SPPHRASE_50,
    pub pSML: windows_sys::core::PWSTR,
    pub pSemanticErrorInfo: *mut SPSEMANTICERRORINFO,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for SPPHRASE_53 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SPPPUT_ARRAY_INDEX: SPPHRASEPROPERTYUNIONTYPE = 1;
pub const SPPPUT_UNUSED: SPPHRASEPROPERTYUNIONTYPE = 0;
pub type SPPRONUNCIATIONFLAGS = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct SPPROPERTYINFO {
    pub pszName: windows_sys::core::PCWSTR,
    pub ulId: u32,
    pub pszValue: windows_sys::core::PCWSTR,
    pub vValue: super::oaidl::VARIANT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for SPPROPERTYINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SPPROP_ADAPTATION_ON: windows_sys::core::PCWSTR = windows_sys::core::w!("AdaptationOn");
pub const SPPROP_COMPLEX_RESPONSE_SPEED: windows_sys::core::PCWSTR = windows_sys::core::w!("ComplexResponseSpeed");
pub const SPPROP_HIGH_CONFIDENCE_THRESHOLD: windows_sys::core::PCWSTR = windows_sys::core::w!("HighConfidenceThreshold");
pub const SPPROP_LOW_CONFIDENCE_THRESHOLD: windows_sys::core::PCWSTR = windows_sys::core::w!("LowConfidenceThreshold");
pub const SPPROP_NORMAL_CONFIDENCE_THRESHOLD: windows_sys::core::PCWSTR = windows_sys::core::w!("NormalConfidenceThreshold");
pub const SPPROP_PERSISTED_BACKGROUND_ADAPTATION: windows_sys::core::PCWSTR = windows_sys::core::w!("PersistedBackgroundAdaptation");
pub const SPPROP_PERSISTED_LANGUAGE_MODEL_ADAPTATION: windows_sys::core::PCWSTR = windows_sys::core::w!("PersistedLanguageModelAdaptation");
pub const SPPROP_RESOURCE_USAGE: windows_sys::core::PCWSTR = windows_sys::core::w!("ResourceUsage");
pub const SPPROP_RESPONSE_SPEED: windows_sys::core::PCWSTR = windows_sys::core::w!("ResponseSpeed");
pub const SPPROP_UX_IS_LISTENING: windows_sys::core::PCWSTR = windows_sys::core::w!("UXIsListening");
pub const SPPR_ALL_ELEMENTS: SPPHRASERNG = -1;
pub const SPPS_Function: SPPARTOFSPEECH = 16384;
pub const SPPS_Interjection: SPPARTOFSPEECH = 20480;
pub const SPPS_LMA: SPPARTOFSPEECH = 28672;
pub const SPPS_Modifier: SPPARTOFSPEECH = 12288;
pub const SPPS_Noncontent: SPPARTOFSPEECH = 24576;
pub const SPPS_NotOverriden: SPPARTOFSPEECH = -1;
pub const SPPS_Noun: SPPARTOFSPEECH = 4096;
pub const SPPS_RESERVED1: SPSHORTCUTTYPE = 12288;
pub const SPPS_RESERVED2: SPSHORTCUTTYPE = 16384;
pub const SPPS_RESERVED3: SPSHORTCUTTYPE = 20480;
pub const SPPS_RESERVED4: SPSHORTCUTTYPE = 61440;
pub const SPPS_SuppressWord: SPPARTOFSPEECH = 61440;
pub const SPPS_Unknown: SPPARTOFSPEECH = 0;
pub const SPPS_Verb: SPPARTOFSPEECH = 8192;
pub const SPRAF_Active: SPCFGRULEATTRIBUTES = 2;
pub const SPRAF_AutoPause: SPCFGRULEATTRIBUTES = 65536;
pub const SPRAF_Dynamic: SPCFGRULEATTRIBUTES = 32;
pub const SPRAF_Export: SPCFGRULEATTRIBUTES = 4;
pub const SPRAF_Import: SPCFGRULEATTRIBUTES = 8;
pub const SPRAF_Interpreter: SPCFGRULEATTRIBUTES = 16;
pub const SPRAF_Root: SPCFGRULEATTRIBUTES = 64;
pub const SPRAF_TopLevel: SPCFGRULEATTRIBUTES = 1;
pub const SPRAF_UserDelimited: SPCFGRULEATTRIBUTES = 131072;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPRECOCONTEXTSTATUS {
    pub eInterference: SPINTERFERENCE,
    pub szRequestTypeOfUI: [u16; 255],
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl Default for SPRECOCONTEXTSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SPRECOEVENTFLAGS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPRECOGNIZERSTATUS {
    pub AudioStatus: SPAUDIOSTATUS,
    pub ullRecognitionStreamPos: u64,
    pub ulStreamNumber: u32,
    pub ulNumActive: u32,
    pub clsidEngine: windows_sys::core::GUID,
    pub cLangIDs: u32,
    pub aLangID: [u16; 20],
    pub ullRecognitionStreamTime: u64,
}
impl Default for SPRECOGNIZERSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Default)]
pub struct SPRECORESULTTIMES {
    pub ftStreamTime: super::minwindef::FILETIME,
    pub ullLength: u64,
    pub dwTickCount: u32,
    pub ullStart: u64,
}
pub type SPRECOSTATE = i32;
pub const SPREF_AutoPause: SPRECOEVENTFLAGS = 1;
pub const SPREF_Emulated: SPRECOEVENTFLAGS = 2;
pub const SPREF_ExtendableParse: SPRECOEVENTFLAGS = 8;
pub const SPREF_FalseRecognition: SPRECOEVENTFLAGS = 64;
pub const SPREF_Hypothesis: SPRECOEVENTFLAGS = 32;
pub const SPREF_ReSent: SPRECOEVENTFLAGS = 16;
pub const SPREF_SMLTimeout: SPRECOEVENTFLAGS = 4;
pub const SPREG_LOCAL_MACHINE_ROOT: windows_sys::core::PCWSTR = windows_sys::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech");
pub const SPREG_SAFE_USER_TOKENS: windows_sys::core::PCWSTR = windows_sys::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\UserTokens");
pub const SPREG_USER_ROOT: windows_sys::core::PCWSTR = windows_sys::core::w!("HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech");
pub const SPRP_NORMAL: u32 = 0;
pub const SPRR_ALL_ELEMENTS: i32 = -1;
pub const SPRST_ACTIVE: SPRECOSTATE = 1;
pub const SPRST_ACTIVE_ALWAYS: SPRECOSTATE = 2;
pub const SPRST_INACTIVE: SPRECOSTATE = 0;
pub const SPRST_INACTIVE_WITH_PURGE: SPRECOSTATE = 3;
pub const SPRST_NUM_STATES: SPRECOSTATE = 4;
pub const SPRS_ACTIVE: SPRULESTATE = 1;
pub const SPRS_ACTIVE_USER_DELIMITED: SPRULESTATE = 4;
pub const SPRS_ACTIVE_WITH_AUTO_PAUSE: SPRULESTATE = 3;
pub const SPRS_DONE: SPRUNSTATE = 1;
pub const SPRS_INACTIVE: SPRULESTATE = 0;
pub const SPRS_IS_SPEAKING: SPRUNSTATE = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPRULE {
    pub pszRuleName: windows_sys::core::PCWSTR,
    pub ulRuleId: u32,
    pub dwAttributes: u32,
}
impl Default for SPRULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SPRULESTATE = i32;
pub type SPRUNSTATE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPSEMANTICERRORINFO {
    pub ulLineNumber: u32,
    pub pszScriptLine: windows_sys::core::PWSTR,
    pub pszSource: windows_sys::core::PWSTR,
    pub pszDescription: windows_sys::core::PWSTR,
    pub hrResultCode: windows_sys::core::HRESULT,
}
impl Default for SPSEMANTICERRORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SPSEMANTICFORMAT = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SPSERIALIZEDEVENT {
    pub _bitfield: SPEVENTENUM,
    pub ulStreamNum: u32,
    pub ullAudioStreamOffset: u64,
    pub SerializedwParam: u32,
    pub SerializedlParam: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SPSERIALIZEDEVENT64 {
    pub _bitfield: SPEVENTENUM,
    pub ulStreamNum: u32,
    pub ullAudioStreamOffset: u64,
    pub SerializedwParam: u64,
    pub SerializedlParam: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SPSERIALIZEDPHRASE {
    pub ulSerializedSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SPSERIALIZEDRESULT {
    pub ulSerializedSize: u32,
}
pub const SPSF_11kHz16BitMono: SPSTREAMFORMAT = 10;
pub const SPSF_11kHz16BitStereo: SPSTREAMFORMAT = 11;
pub const SPSF_11kHz8BitMono: SPSTREAMFORMAT = 8;
pub const SPSF_11kHz8BitStereo: SPSTREAMFORMAT = 9;
pub const SPSF_12kHz16BitMono: SPSTREAMFORMAT = 14;
pub const SPSF_12kHz16BitStereo: SPSTREAMFORMAT = 15;
pub const SPSF_12kHz8BitMono: SPSTREAMFORMAT = 12;
pub const SPSF_12kHz8BitStereo: SPSTREAMFORMAT = 13;
pub const SPSF_16kHz16BitMono: SPSTREAMFORMAT = 18;
pub const SPSF_16kHz16BitStereo: SPSTREAMFORMAT = 19;
pub const SPSF_16kHz8BitMono: SPSTREAMFORMAT = 16;
pub const SPSF_16kHz8BitStereo: SPSTREAMFORMAT = 17;
pub const SPSF_22kHz16BitMono: SPSTREAMFORMAT = 22;
pub const SPSF_22kHz16BitStereo: SPSTREAMFORMAT = 23;
pub const SPSF_22kHz8BitMono: SPSTREAMFORMAT = 20;
pub const SPSF_22kHz8BitStereo: SPSTREAMFORMAT = 21;
pub const SPSF_24kHz16BitMono: SPSTREAMFORMAT = 26;
pub const SPSF_24kHz16BitStereo: SPSTREAMFORMAT = 27;
pub const SPSF_24kHz8BitMono: SPSTREAMFORMAT = 24;
pub const SPSF_24kHz8BitStereo: SPSTREAMFORMAT = 25;
pub const SPSF_32kHz16BitMono: SPSTREAMFORMAT = 30;
pub const SPSF_32kHz16BitStereo: SPSTREAMFORMAT = 31;
pub const SPSF_32kHz8BitMono: SPSTREAMFORMAT = 28;
pub const SPSF_32kHz8BitStereo: SPSTREAMFORMAT = 29;
pub const SPSF_44kHz16BitMono: SPSTREAMFORMAT = 34;
pub const SPSF_44kHz16BitStereo: SPSTREAMFORMAT = 35;
pub const SPSF_44kHz8BitMono: SPSTREAMFORMAT = 32;
pub const SPSF_44kHz8BitStereo: SPSTREAMFORMAT = 33;
pub const SPSF_48kHz16BitMono: SPSTREAMFORMAT = 38;
pub const SPSF_48kHz16BitStereo: SPSTREAMFORMAT = 39;
pub const SPSF_48kHz8BitMono: SPSTREAMFORMAT = 36;
pub const SPSF_48kHz8BitStereo: SPSTREAMFORMAT = 37;
pub const SPSF_8kHz16BitMono: SPSTREAMFORMAT = 6;
pub const SPSF_8kHz16BitStereo: SPSTREAMFORMAT = 7;
pub const SPSF_8kHz8BitMono: SPSTREAMFORMAT = 4;
pub const SPSF_8kHz8BitStereo: SPSTREAMFORMAT = 5;
pub const SPSF_ADPCM_11kHzMono: SPSTREAMFORMAT = 59;
pub const SPSF_ADPCM_11kHzStereo: SPSTREAMFORMAT = 60;
pub const SPSF_ADPCM_22kHzMono: SPSTREAMFORMAT = 61;
pub const SPSF_ADPCM_22kHzStereo: SPSTREAMFORMAT = 62;
pub const SPSF_ADPCM_44kHzMono: SPSTREAMFORMAT = 63;
pub const SPSF_ADPCM_44kHzStereo: SPSTREAMFORMAT = 64;
pub const SPSF_ADPCM_8kHzMono: SPSTREAMFORMAT = 57;
pub const SPSF_ADPCM_8kHzStereo: SPSTREAMFORMAT = 58;
pub const SPSF_CCITT_ALaw_11kHzMono: SPSTREAMFORMAT = 43;
pub const SPSF_CCITT_ALaw_11kHzStereo: SPSTREAMFORMAT = 44;
pub const SPSF_CCITT_ALaw_22kHzMono: SPSTREAMFORMAT = 45;
pub const SPSF_CCITT_ALaw_22kHzStereo: SPSTREAMFORMAT = 46;
pub const SPSF_CCITT_ALaw_44kHzMono: SPSTREAMFORMAT = 47;
pub const SPSF_CCITT_ALaw_44kHzStereo: SPSTREAMFORMAT = 48;
pub const SPSF_CCITT_ALaw_8kHzMono: SPSTREAMFORMAT = 41;
pub const SPSF_CCITT_ALaw_8kHzStereo: SPSTREAMFORMAT = 42;
pub const SPSF_CCITT_uLaw_11kHzMono: SPSTREAMFORMAT = 51;
pub const SPSF_CCITT_uLaw_11kHzStereo: SPSTREAMFORMAT = 52;
pub const SPSF_CCITT_uLaw_22kHzMono: SPSTREAMFORMAT = 53;
pub const SPSF_CCITT_uLaw_22kHzStereo: SPSTREAMFORMAT = 54;
pub const SPSF_CCITT_uLaw_44kHzMono: SPSTREAMFORMAT = 55;
pub const SPSF_CCITT_uLaw_44kHzStereo: SPSTREAMFORMAT = 56;
pub const SPSF_CCITT_uLaw_8kHzMono: SPSTREAMFORMAT = 49;
pub const SPSF_CCITT_uLaw_8kHzStereo: SPSTREAMFORMAT = 50;
pub const SPSF_Default: SPSTREAMFORMAT = -1;
pub const SPSF_ExtendedAudioFormat: SPSTREAMFORMAT = 3;
pub const SPSF_GSM610_11kHzMono: SPSTREAMFORMAT = 66;
pub const SPSF_GSM610_22kHzMono: SPSTREAMFORMAT = 67;
pub const SPSF_GSM610_44kHzMono: SPSTREAMFORMAT = 68;
pub const SPSF_GSM610_8kHzMono: SPSTREAMFORMAT = 65;
pub const SPSF_NUM_FORMATS: SPSTREAMFORMAT = 69;
pub const SPSF_NoAssignedFormat: SPSTREAMFORMAT = 0;
pub const SPSF_NonStandardFormat: SPSTREAMFORMAT = 2;
pub const SPSF_Text: SPSTREAMFORMAT = 1;
pub const SPSF_TrueSpeech_8kHz1BitMono: SPSTREAMFORMAT = 40;
pub const SPSFunction: SpeechPartOfSpeech = 16384;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPSHORTCUTPAIR {
    pub pNextSHORTCUTPAIR: *mut Self,
    pub LangID: u16,
    pub shType: SPSHORTCUTTYPE,
    pub pszDisplay: windows_sys::core::PWSTR,
    pub pszSpoken: windows_sys::core::PWSTR,
}
impl Default for SPSHORTCUTPAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPSHORTCUTPAIRLIST {
    pub ulSize: u32,
    pub pvBuffer: *mut u8,
    pub pFirstShortcutPair: *mut SPSHORTCUTPAIR,
}
impl Default for SPSHORTCUTPAIRLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SPSHORTCUTTYPE = i32;
pub const SPSHT_EMAIL: SPSHORTCUTTYPE = 4096;
pub const SPSHT_NotOverriden: SPSHORTCUTTYPE = -1;
pub const SPSHT_OTHER: SPSHORTCUTTYPE = 8192;
pub const SPSHT_Unknown: SPSHORTCUTTYPE = 0;
pub const SPSInterjection: SpeechPartOfSpeech = 20480;
pub const SPSLMA: SpeechPartOfSpeech = 28672;
pub const SPSMF_SAPI_PROPERTIES: SPSEMANTICFORMAT = 0;
pub const SPSMF_SRGS_SAPIPROPERTIES: SPSEMANTICFORMAT = 2;
pub const SPSMF_SRGS_SEMANTICINTERPRETATION_MS: SPSEMANTICFORMAT = 1;
pub const SPSMF_SRGS_SEMANTICINTERPRETATION_W3C: SPSEMANTICFORMAT = 8;
pub const SPSMF_UPS: SPSEMANTICFORMAT = 4;
pub const SPSModifier: SpeechPartOfSpeech = 12288;
pub const SPSNotOverriden: SpeechPartOfSpeech = -1;
pub const SPSNoun: SpeechPartOfSpeech = 4096;
pub const SPSSuppressWord: SpeechPartOfSpeech = 61440;
pub type SPSTATEHANDLE = *mut core::ffi::c_void;
pub type SPSTREAMFORMAT = i32;
pub type SPSTREAMFORMATTYPE = i32;
pub const SPSUnknown: SpeechPartOfSpeech = 0;
pub const SPSVerb: SpeechPartOfSpeech = 8192;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SPTEXTSELECTIONINFO {
    pub ulStartActiveOffset: u32,
    pub cchActiveChars: u32,
    pub ulStartSelection: u32,
    pub cchSelection: u32,
}
pub const SPTOKENKEY_ATTRIBUTES: windows_sys::core::PCWSTR = windows_sys::core::w!("Attributes");
pub const SPTOKENKEY_AUDIO_LATENCY_TRUNCATE: windows_sys::core::PCWSTR = windows_sys::core::w!("LatencyTruncateThreshold");
pub const SPTOKENKEY_AUDIO_LATENCY_UPDATE_INTERVAL: windows_sys::core::PCWSTR = windows_sys::core::w!("LatencyUpdateInterval");
pub const SPTOKENKEY_AUDIO_LATENCY_WARNING: windows_sys::core::PCWSTR = windows_sys::core::w!("LatencyWarningThreshold");
pub const SPTOKENKEY_FILES: windows_sys::core::PCWSTR = windows_sys::core::w!("Files");
pub const SPTOKENKEY_RETAINEDAUDIO: windows_sys::core::PCWSTR = windows_sys::core::w!("SecondsPerRetainedAudioEvent");
pub const SPTOKENKEY_UI: windows_sys::core::PCWSTR = windows_sys::core::w!("UI");
pub const SPTOKENVALUE_CLSID: windows_sys::core::PCWSTR = windows_sys::core::w!("CLSID");
pub const SPTOPIC_SPELLING: windows_sys::core::PCWSTR = windows_sys::core::w!("Spelling");
pub type SPVACTIONS = i32;
pub type SPVALUETYPE = i32;
pub const SPVA_Bookmark: SPVACTIONS = 3;
pub const SPVA_ParseUnknownTag: SPVACTIONS = 6;
pub const SPVA_Pronounce: SPVACTIONS = 2;
pub const SPVA_Section: SPVACTIONS = 5;
pub const SPVA_Silence: SPVACTIONS = 1;
pub const SPVA_Speak: SPVACTIONS = 0;
pub const SPVA_SpellOut: SPVACTIONS = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPVCONTEXT {
    pub pCategory: windows_sys::core::PCWSTR,
    pub pBefore: windows_sys::core::PCWSTR,
    pub pAfter: windows_sys::core::PCWSTR,
}
impl Default for SPVCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SPVFEATURE = i32;
pub const SPVFEATURE_EMPHASIS: SPVFEATURE = 2;
pub const SPVFEATURE_STRESSED: SPVFEATURE = 1;
pub type SPVISEMES = i32;
pub type SPVLIMITS = i32;
pub const SPVOICECATEGORY_TTSRATE: windows_sys::core::PCWSTR = windows_sys::core::w!("DefaultTTSRate");
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SPVOICESTATUS {
    pub ulCurrentStream: u32,
    pub ulLastStreamQueued: u32,
    pub hrLastResult: windows_sys::core::HRESULT,
    pub dwRunningState: u32,
    pub ulInputWordPos: u32,
    pub ulInputWordLen: u32,
    pub ulInputSentPos: u32,
    pub ulInputSentLen: u32,
    pub lBookmarkId: i32,
    pub PhonemeId: SPPHONEID,
    pub VisemeId: SPVISEMES,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SPVPITCH {
    pub MiddleAdj: i32,
    pub RangeAdj: i32,
}
pub type SPVPRIORITY = i32;
pub const SPVPRI_ALERT: SPVPRIORITY = 1;
pub const SPVPRI_NORMAL: SPVPRIORITY = 0;
pub const SPVPRI_OVER: SPVPRIORITY = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPVSTATE {
    pub eAction: SPVACTIONS,
    pub LangID: u16,
    pub wReserved: u16,
    pub EmphAdj: i32,
    pub RateAdj: i32,
    pub Volume: u32,
    pub PitchAdj: SPVPITCH,
    pub SilenceMSecs: u32,
    pub pPhoneIds: *mut SPPHONEID,
    pub ePartOfSpeech: SPPARTOFSPEECH,
    pub Context: SPVCONTEXT,
}
impl Default for SPVSTATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SPWF_INPUT: SPSTREAMFORMATTYPE = 0;
pub const SPWF_SRENGINE: SPSTREAMFORMATTYPE = 1;
pub const SPWILDCARD: windows_sys::core::PCWSTR = windows_sys::core::w!("...");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPWORD {
    pub pNextWord: *mut Self,
    pub LangID: u16,
    pub wReserved: u16,
    pub eWordType: SPWORDTYPE,
    pub pszWord: windows_sys::core::PWSTR,
    pub pFirstWordPronunciation: *mut SPWORDPRONUNCIATION,
}
impl Default for SPWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPWORDLIST {
    pub ulSize: u32,
    pub pvBuffer: *mut u8,
    pub pFirstWord: *mut SPWORD,
}
impl Default for SPWORDLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SPWORDPRONOUNCEABLE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPWORDPRONUNCIATION {
    pub pNextWordPronunciation: *mut Self,
    pub eLexiconType: SPLEXICONTYPE,
    pub LangID: u16,
    pub wPronunciationFlags: u16,
    pub ePartOfSpeech: SPPARTOFSPEECH,
    pub szPronunciation: [SPPHONEID; 1],
}
impl Default for SPWORDPRONUNCIATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPWORDPRONUNCIATIONLIST {
    pub ulSize: u32,
    pub pvBuffer: *mut u8,
    pub pFirstWordPronunciation: *mut SPWORDPRONUNCIATION,
}
impl Default for SPWORDPRONUNCIATIONLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SPWORDTYPE = i32;
pub const SPWP_KNOWN_WORD_PRONOUNCEABLE: SPWORDPRONOUNCEABLE = 2;
pub const SPWP_UNKNOWN_WORD_PRONOUNCEABLE: SPWORDPRONOUNCEABLE = 1;
pub const SPWP_UNKNOWN_WORD_UNPRONOUNCEABLE: SPWORDPRONOUNCEABLE = 0;
pub const SPWT_DISPLAY: SPGRAMMARWORDTYPE = 0;
pub const SPWT_LEXICAL: SPGRAMMARWORDTYPE = 1;
pub const SPWT_LEXICAL_NO_SPECIAL_CHARS: SPGRAMMARWORDTYPE = 3;
pub const SPWT_PRONUNCIATION: SPGRAMMARWORDTYPE = 2;
pub type SPXMLRESULTOPTIONS = i32;
pub const SPXRO_Alternates_SML: SPXMLRESULTOPTIONS = 1;
pub const SPXRO_SML: SPXMLRESULTOPTIONS = 0;
pub const SP_EMULATE_RESULT: u32 = 1073741824;
pub const SP_GETWHOLEPHRASE: i32 = -1;
pub const SP_HIGH_CONFIDENCE: u32 = 1;
pub const SP_LOW_CONFIDENCE: i32 = -1;
pub const SP_MAX_LANGIDS: u32 = 20;
pub const SP_MAX_PRON_LENGTH: u32 = 384;
pub const SP_MAX_WORD_LENGTH: u32 = 128;
pub const SP_NORMAL_CONFIDENCE: u32 = 0;
pub const SP_STREAMPOS_ASAP: u32 = 0;
pub const SP_STREAMPOS_REALTIME: i32 = -1;
pub const SP_VISEME_0: SPVISEMES = 0;
pub const SP_VISEME_1: SPVISEMES = 1;
pub const SP_VISEME_10: SPVISEMES = 10;
pub const SP_VISEME_11: SPVISEMES = 11;
pub const SP_VISEME_12: SPVISEMES = 12;
pub const SP_VISEME_13: SPVISEMES = 13;
pub const SP_VISEME_14: SPVISEMES = 14;
pub const SP_VISEME_15: SPVISEMES = 15;
pub const SP_VISEME_16: SPVISEMES = 16;
pub const SP_VISEME_17: SPVISEMES = 17;
pub const SP_VISEME_18: SPVISEMES = 18;
pub const SP_VISEME_19: SPVISEMES = 19;
pub const SP_VISEME_2: SPVISEMES = 2;
pub const SP_VISEME_20: SPVISEMES = 20;
pub const SP_VISEME_21: SPVISEMES = 21;
pub const SP_VISEME_3: SPVISEMES = 3;
pub const SP_VISEME_4: SPVISEMES = 4;
pub const SP_VISEME_5: SPVISEMES = 5;
pub const SP_VISEME_6: SPVISEMES = 6;
pub const SP_VISEME_7: SPVISEMES = 7;
pub const SP_VISEME_8: SPVISEMES = 8;
pub const SP_VISEME_9: SPVISEMES = 9;
pub const SRADefaultToActive: SpeechRuleAttributes = 2;
pub const SRADynamic: SpeechRuleAttributes = 32;
pub const SRAExport: SpeechRuleAttributes = 4;
pub const SRAImport: SpeechRuleAttributes = 8;
pub const SRAInterpreter: SpeechRuleAttributes = 16;
pub const SRAONone: SpeechRetainedAudioOptions = 0;
pub const SRAORetainAudio: SpeechRetainedAudioOptions = 1;
pub const SRARoot: SpeechRuleAttributes = 64;
pub const SRATopLevel: SpeechRuleAttributes = 1;
pub const SRCS_Disabled: SpeechRecoContextState = 0;
pub const SRCS_Enabled: SpeechRecoContextState = 1;
pub const SREAdaptation: SpeechRecoEvents = 8192;
pub const SREAllEvents: SpeechRecoEvents = 393215;
pub const SREAudioLevel: SpeechRecoEvents = 65536;
pub const SREBookmark: SpeechRecoEvents = 64;
pub const SREFalseRecognition: SpeechRecoEvents = 512;
pub const SREHypothesis: SpeechRecoEvents = 32;
pub const SREInterference: SpeechRecoEvents = 1024;
pub const SREPhraseStart: SpeechRecoEvents = 8;
pub const SREPrivate: SpeechRecoEvents = 262144;
pub const SREPropertyNumChange: SpeechRecoEvents = 128;
pub const SREPropertyStringChange: SpeechRecoEvents = 256;
pub const SRERecoOtherContext: SpeechRecoEvents = 32768;
pub const SRERecognition: SpeechRecoEvents = 16;
pub const SRERequestUI: SpeechRecoEvents = 2048;
pub const SRESoundEnd: SpeechRecoEvents = 4;
pub const SRESoundStart: SpeechRecoEvents = 2;
pub const SREStateChange: SpeechRecoEvents = 4096;
pub const SREStreamEnd: SpeechRecoEvents = 1;
pub const SREStreamStart: SpeechRecoEvents = 16384;
pub const SRSActive: SpeechRecognizerState = 1;
pub const SRSActiveAlways: SpeechRecognizerState = 2;
pub const SRSEDone: SpeechRunState = 1;
pub const SRSEIsSpeaking: SpeechRunState = 2;
pub const SRSInactive: SpeechRecognizerState = 0;
pub const SRSInactiveWithPurge: SpeechRecognizerState = 3;
pub const SRTAutopause: SpeechRecognitionType = 1;
pub const SRTEmulated: SpeechRecognitionType = 2;
pub const SRTExtendableParse: SpeechRecognitionType = 8;
pub const SRTReSent: SpeechRecognitionType = 16;
pub const SRTSMLTimeout: SpeechRecognitionType = 4;
pub const SRTStandard: SpeechRecognitionType = 0;
pub const SSFMCreate: SpeechStreamFileMode = 2;
pub const SSFMCreateForWrite: SpeechStreamFileMode = 3;
pub const SSFMOpenForRead: SpeechStreamFileMode = 0;
pub const SSFMOpenReadWrite: SpeechStreamFileMode = 1;
pub const SSSPTRelativeToCurrentPosition: SpeechStreamSeekPositionType = 1;
pub const SSSPTRelativeToEnd: SpeechStreamSeekPositionType = 2;
pub const SSSPTRelativeToStart: SpeechStreamSeekPositionType = 0;
pub const SSTTDictation: SpeechSpecialTransitionType = 2;
pub const SSTTTextBuffer: SpeechSpecialTransitionType = 3;
pub const SSTTWildcard: SpeechSpecialTransitionType = 1;
pub const STCAll: SpeechTokenContext = 23;
pub const STCInprocHandler: SpeechTokenContext = 2;
pub const STCInprocServer: SpeechTokenContext = 1;
pub const STCLocalServer: SpeechTokenContext = 4;
pub const STCRemoteServer: SpeechTokenContext = 16;
pub const STSF_AppData: SpeechTokenShellFolder = 26;
pub const STSF_CommonAppData: SpeechTokenShellFolder = 35;
pub const STSF_FlagCreate: SpeechTokenShellFolder = 32768;
pub const STSF_LocalAppData: SpeechTokenShellFolder = 28;
pub const SVEAllEvents: SpeechVoiceEvents = 33790;
pub const SVEAudioLevel: SpeechVoiceEvents = 512;
pub const SVEBookmark: SpeechVoiceEvents = 16;
pub const SVEEndInputStream: SpeechVoiceEvents = 4;
pub const SVEPhoneme: SpeechVoiceEvents = 64;
pub const SVEPrivate: SpeechVoiceEvents = 32768;
pub const SVESentenceBoundary: SpeechVoiceEvents = 128;
pub const SVEStartInputStream: SpeechVoiceEvents = 2;
pub const SVEViseme: SpeechVoiceEvents = 256;
pub const SVEVoiceChange: SpeechVoiceEvents = 8;
pub const SVEWordBoundary: SpeechVoiceEvents = 32;
pub const SVF_Emphasis: SpeechVisemeFeature = 2;
pub const SVF_None: SpeechVisemeFeature = 0;
pub const SVF_Stressed: SpeechVisemeFeature = 1;
pub const SVPAlert: SpeechVoicePriority = 1;
pub const SVPNormal: SpeechVoicePriority = 0;
pub const SVPOver: SpeechVoicePriority = 2;
pub const SVP_0: SpeechVisemeType = 0;
pub const SVP_1: SpeechVisemeType = 1;
pub const SVP_10: SpeechVisemeType = 10;
pub const SVP_11: SpeechVisemeType = 11;
pub const SVP_12: SpeechVisemeType = 12;
pub const SVP_13: SpeechVisemeType = 13;
pub const SVP_14: SpeechVisemeType = 14;
pub const SVP_15: SpeechVisemeType = 15;
pub const SVP_16: SpeechVisemeType = 16;
pub const SVP_17: SpeechVisemeType = 17;
pub const SVP_18: SpeechVisemeType = 18;
pub const SVP_19: SpeechVisemeType = 19;
pub const SVP_2: SpeechVisemeType = 2;
pub const SVP_20: SpeechVisemeType = 20;
pub const SVP_21: SpeechVisemeType = 21;
pub const SVP_3: SpeechVisemeType = 3;
pub const SVP_4: SpeechVisemeType = 4;
pub const SVP_5: SpeechVisemeType = 5;
pub const SVP_6: SpeechVisemeType = 6;
pub const SVP_7: SpeechVisemeType = 7;
pub const SVP_8: SpeechVisemeType = 8;
pub const SVP_9: SpeechVisemeType = 9;
pub const SVSFDefault: SpeechVoiceSpeakFlags = 0;
pub const SVSFIsFilename: SpeechVoiceSpeakFlags = 4;
pub const SVSFIsNotXML: SpeechVoiceSpeakFlags = 16;
pub const SVSFIsXML: SpeechVoiceSpeakFlags = 8;
pub const SVSFNLPMask: SpeechVoiceSpeakFlags = 64;
pub const SVSFNLPSpeakPunc: SpeechVoiceSpeakFlags = 64;
pub const SVSFParseAutodetect: SpeechVoiceSpeakFlags = 0;
pub const SVSFParseMask: SpeechVoiceSpeakFlags = 384;
pub const SVSFParseSapi: SpeechVoiceSpeakFlags = 128;
pub const SVSFParseSsml: SpeechVoiceSpeakFlags = 256;
pub const SVSFPersistXML: SpeechVoiceSpeakFlags = 32;
pub const SVSFPurgeBeforeSpeak: SpeechVoiceSpeakFlags = 2;
pub const SVSFUnusedFlags: SpeechVoiceSpeakFlags = -512;
pub const SVSFVoiceMask: SpeechVoiceSpeakFlags = 511;
pub const SVSFlagsAsync: SpeechVoiceSpeakFlags = 1;
pub const SWPKnownWordPronounceable: SpeechWordPronounceable = 2;
pub const SWPUnknownWordPronounceable: SpeechWordPronounceable = 1;
pub const SWPUnknownWordUnpronounceable: SpeechWordPronounceable = 0;
pub const SWTAdded: SpeechWordType = 1;
pub const SWTDeleted: SpeechWordType = 2;
pub const SpAudioFormat: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9ef96870_e160_4792_820d_48cf0649e4ec);
pub const SpCompressedLexicon: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x90903716_2f42_11d3_9c26_00c04f8ef87c);
pub const SpCustomStream: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8dbef13f_1948_4aa8_8cf0_048eebed95d8);
pub const SpFileStream: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x947812b3_2ae1_4644_ba86_9e90ded7ec91);
pub const SpInProcRecoContext: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x73ad6842_ace0_45e8_a4dd_8795881a2c2a);
pub const SpInprocRecognizer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x41b89b6b_9399_11d2_9623_00c04f8ee628);
pub const SpLexicon: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0655e396_25d0_11d3_9c26_00c04f8ef87c);
pub const SpMMAudioEnum: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xab1890a0_e91f_11d2_bb91_00c04f8ee6c0);
pub const SpMMAudioIn: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcf3d2e50_53f2_11d2_960c_00c04f8ee628);
pub const SpMMAudioOut: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa8c680eb_3d32_11d2_9ee7_00c04f797396);
pub const SpMemoryStream: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5fb7ef7d_dff4_468a_b6b7_2fcbd188f994);
pub const SpNotifyTranslator: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe2ae5372_5d40_11d2_960e_00c04f8ee628);
pub const SpNullPhoneConverter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x455f24e9_7396_4a16_9715_7c0fdbe3efe3);
pub const SpObjectToken: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xef411752_3736_4cb4_9c8c_8ef4ccb58efe);
pub const SpObjectTokenCategory: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa910187f_0c7a_45ac_92cc_59edafb77b53);
pub const SpPhoneConverter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9185f743_1143_4c28_86b5_bff14f20e5c8);
pub const SpPhoneticAlphabetConverter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4f414126_dfe3_4629_99ee_797978317ead);
pub const SpPhraseInfoBuilder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc23fc28d_c55f_4720_8b32_91f73c2bd5d1);
pub const SpResourceManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96749373_3391_11d2_9ee3_00c04f797396);
pub const SpSharedRecoContext: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x47206204_5eca_11d2_960f_00c04f8ee628);
pub const SpSharedRecognizer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3bee4890_4fe9_4a37_8c1e_5e7e12791c1f);
pub const SpShortcut: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0d722f1a_9fcf_4e62_96d8_6df8f01a26aa);
pub const SpStream: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x715d9c59_4442_11d2_9605_00c04f8ee628);
pub const SpStreamFormatConverter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7013943a_e2ec_11d2_a086_00c04f8ef9b5);
pub const SpTextSelectionInformation: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0f92030a_cbfd_4ab8_a164_ff5985547ff6);
pub const SpUnCompressedLexicon: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc9e37c15_df92_4727_85d6_72e5eeb6995a);
pub const SpVoice: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x96749377_3391_11d2_9ee3_00c04f797396);
pub const SpWaveFormatEx: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc79a574c_63be_44b9_801f_283f87f898be);
pub type SpeechAudioFormatType = i32;
pub type SpeechAudioState = i32;
pub type SpeechBookmarkOptions = i32;
pub type SpeechDataKeyLocation = i32;
pub type SpeechDiscardType = i32;
pub type SpeechDisplayAttributes = i32;
pub type SpeechEmulationCompareFlags = i32;
pub type SpeechEngineConfidence = i32;
pub type SpeechFormatType = i32;
pub type SpeechGrammarRuleStateTransitionType = i32;
pub type SpeechGrammarState = i32;
pub type SpeechGrammarWordType = i32;
pub type SpeechInterference = i32;
pub type SpeechLanguageId = i32;
pub type SpeechLexiconType = i32;
pub type SpeechLoadOption = i32;
pub type SpeechPartOfSpeech = i32;
pub type SpeechRecoContextState = i32;
pub type SpeechRecoEvents = i32;
pub type SpeechRecognitionType = i32;
pub type SpeechRecognizerState = i32;
pub type SpeechRetainedAudioOptions = i32;
pub type SpeechRuleAttributes = i32;
pub type SpeechRuleState = i32;
pub type SpeechRunState = i32;
pub type SpeechSpecialTransitionType = i32;
pub type SpeechStreamFileMode = i32;
pub type SpeechStreamSeekPositionType = i32;
pub type SpeechTokenContext = i32;
pub type SpeechTokenShellFolder = i32;
pub type SpeechVisemeFeature = i32;
pub type SpeechVisemeType = i32;
pub type SpeechVoiceEvents = i32;
pub type SpeechVoicePriority = i32;
pub type SpeechVoiceSpeakFlags = i32;
pub type SpeechWordPronounceable = i32;
pub type SpeechWordType = i32;
pub const Subsequence: SPMATCHINGMODE = 1;
pub const SubsequenceContentRequired: SPMATCHINGMODE = 5;
pub const eLEXTYPE_APP: SPLEXICONTYPE = 2;
pub const eLEXTYPE_LETTERTOSOUND: SPLEXICONTYPE = 8;
pub const eLEXTYPE_MORPHOLOGY: SPLEXICONTYPE = 16;
pub const eLEXTYPE_PRIVATE1: SPLEXICONTYPE = 4096;
pub const eLEXTYPE_PRIVATE10: SPLEXICONTYPE = 2097152;
pub const eLEXTYPE_PRIVATE11: SPLEXICONTYPE = 4194304;
pub const eLEXTYPE_PRIVATE12: SPLEXICONTYPE = 8388608;
pub const eLEXTYPE_PRIVATE13: SPLEXICONTYPE = 16777216;
pub const eLEXTYPE_PRIVATE14: SPLEXICONTYPE = 33554432;
pub const eLEXTYPE_PRIVATE15: SPLEXICONTYPE = 67108864;
pub const eLEXTYPE_PRIVATE16: SPLEXICONTYPE = 134217728;
pub const eLEXTYPE_PRIVATE17: SPLEXICONTYPE = 268435456;
pub const eLEXTYPE_PRIVATE18: SPLEXICONTYPE = 536870912;
pub const eLEXTYPE_PRIVATE19: SPLEXICONTYPE = 1073741824;
pub const eLEXTYPE_PRIVATE2: SPLEXICONTYPE = 8192;
pub const eLEXTYPE_PRIVATE20: SPLEXICONTYPE = -2147483648;
pub const eLEXTYPE_PRIVATE3: SPLEXICONTYPE = 16384;
pub const eLEXTYPE_PRIVATE4: SPLEXICONTYPE = 32768;
pub const eLEXTYPE_PRIVATE5: SPLEXICONTYPE = 65536;
pub const eLEXTYPE_PRIVATE6: SPLEXICONTYPE = 131072;
pub const eLEXTYPE_PRIVATE7: SPLEXICONTYPE = 262144;
pub const eLEXTYPE_PRIVATE8: SPLEXICONTYPE = 524288;
pub const eLEXTYPE_PRIVATE9: SPLEXICONTYPE = 1048576;
pub const eLEXTYPE_RESERVED10: SPLEXICONTYPE = 2048;
pub const eLEXTYPE_RESERVED4: SPLEXICONTYPE = 32;
pub const eLEXTYPE_RESERVED6: SPLEXICONTYPE = 128;
pub const eLEXTYPE_RESERVED7: SPLEXICONTYPE = 256;
pub const eLEXTYPE_RESERVED8: SPLEXICONTYPE = 512;
pub const eLEXTYPE_RESERVED9: SPLEXICONTYPE = 1024;
pub const eLEXTYPE_USER: SPLEXICONTYPE = 1;
pub const eLEXTYPE_USER_SHORTCUT: SPLEXICONTYPE = 64;
pub const eLEXTYPE_VENDORLEXICON: SPLEXICONTYPE = 4;
pub const ePRONFLAG_USED: SPPRONUNCIATIONFLAGS = 1;
pub const eWORDTYPE_ADDED: SPWORDTYPE = 1;
pub const eWORDTYPE_DELETED: SPWORDTYPE = 2;
