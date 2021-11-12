#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
pub const DEFAULT_WEIGHT: u32 = 1u32;
#[repr(transparent)]
pub struct DISPIDSPRG(pub i32);
pub const DISPID_SRGId: DISPIDSPRG = DISPIDSPRG(1i32);
pub const DISPID_SRGRecoContext: DISPIDSPRG = DISPIDSPRG(2i32);
pub const DISPID_SRGState: DISPIDSPRG = DISPIDSPRG(3i32);
pub const DISPID_SRGRules: DISPIDSPRG = DISPIDSPRG(4i32);
pub const DISPID_SRGReset: DISPIDSPRG = DISPIDSPRG(5i32);
pub const DISPID_SRGCommit: DISPIDSPRG = DISPIDSPRG(6i32);
pub const DISPID_SRGCmdLoadFromFile: DISPIDSPRG = DISPIDSPRG(7i32);
pub const DISPID_SRGCmdLoadFromObject: DISPIDSPRG = DISPIDSPRG(8i32);
pub const DISPID_SRGCmdLoadFromResource: DISPIDSPRG = DISPIDSPRG(9i32);
pub const DISPID_SRGCmdLoadFromMemory: DISPIDSPRG = DISPIDSPRG(10i32);
pub const DISPID_SRGCmdLoadFromProprietaryGrammar: DISPIDSPRG = DISPIDSPRG(11i32);
pub const DISPID_SRGCmdSetRuleState: DISPIDSPRG = DISPIDSPRG(12i32);
pub const DISPID_SRGCmdSetRuleIdState: DISPIDSPRG = DISPIDSPRG(13i32);
pub const DISPID_SRGDictationLoad: DISPIDSPRG = DISPIDSPRG(14i32);
pub const DISPID_SRGDictationUnload: DISPIDSPRG = DISPIDSPRG(15i32);
pub const DISPID_SRGDictationSetState: DISPIDSPRG = DISPIDSPRG(16i32);
pub const DISPID_SRGSetWordSequenceData: DISPIDSPRG = DISPIDSPRG(17i32);
pub const DISPID_SRGSetTextSelection: DISPIDSPRG = DISPIDSPRG(18i32);
pub const DISPID_SRGIsPronounceable: DISPIDSPRG = DISPIDSPRG(19i32);
#[repr(transparent)]
pub struct DISPIDSPTSI(pub i32);
pub const DISPIDSPTSI_ActiveOffset: DISPIDSPTSI = DISPIDSPTSI(1i32);
pub const DISPIDSPTSI_ActiveLength: DISPIDSPTSI = DISPIDSPTSI(2i32);
pub const DISPIDSPTSI_SelectionOffset: DISPIDSPTSI = DISPIDSPTSI(3i32);
pub const DISPIDSPTSI_SelectionLength: DISPIDSPTSI = DISPIDSPTSI(4i32);
#[repr(transparent)]
pub struct DISPID_SpeechAudio(pub i32);
pub const DISPID_SAStatus: DISPID_SpeechAudio = DISPID_SpeechAudio(200i32);
pub const DISPID_SABufferInfo: DISPID_SpeechAudio = DISPID_SpeechAudio(201i32);
pub const DISPID_SADefaultFormat: DISPID_SpeechAudio = DISPID_SpeechAudio(202i32);
pub const DISPID_SAVolume: DISPID_SpeechAudio = DISPID_SpeechAudio(203i32);
pub const DISPID_SABufferNotifySize: DISPID_SpeechAudio = DISPID_SpeechAudio(204i32);
pub const DISPID_SAEventHandle: DISPID_SpeechAudio = DISPID_SpeechAudio(205i32);
pub const DISPID_SASetState: DISPID_SpeechAudio = DISPID_SpeechAudio(206i32);
#[repr(transparent)]
pub struct DISPID_SpeechAudioBufferInfo(pub i32);
pub const DISPID_SABIMinNotification: DISPID_SpeechAudioBufferInfo = DISPID_SpeechAudioBufferInfo(1i32);
pub const DISPID_SABIBufferSize: DISPID_SpeechAudioBufferInfo = DISPID_SpeechAudioBufferInfo(2i32);
pub const DISPID_SABIEventBias: DISPID_SpeechAudioBufferInfo = DISPID_SpeechAudioBufferInfo(3i32);
#[repr(transparent)]
pub struct DISPID_SpeechAudioFormat(pub i32);
pub const DISPID_SAFType: DISPID_SpeechAudioFormat = DISPID_SpeechAudioFormat(1i32);
pub const DISPID_SAFGuid: DISPID_SpeechAudioFormat = DISPID_SpeechAudioFormat(2i32);
pub const DISPID_SAFGetWaveFormatEx: DISPID_SpeechAudioFormat = DISPID_SpeechAudioFormat(3i32);
pub const DISPID_SAFSetWaveFormatEx: DISPID_SpeechAudioFormat = DISPID_SpeechAudioFormat(4i32);
#[repr(transparent)]
pub struct DISPID_SpeechAudioStatus(pub i32);
pub const DISPID_SASFreeBufferSpace: DISPID_SpeechAudioStatus = DISPID_SpeechAudioStatus(1i32);
pub const DISPID_SASNonBlockingIO: DISPID_SpeechAudioStatus = DISPID_SpeechAudioStatus(2i32);
pub const DISPID_SASState: DISPID_SpeechAudioStatus = DISPID_SpeechAudioStatus(3i32);
pub const DISPID_SASCurrentSeekPosition: DISPID_SpeechAudioStatus = DISPID_SpeechAudioStatus(4i32);
pub const DISPID_SASCurrentDevicePosition: DISPID_SpeechAudioStatus = DISPID_SpeechAudioStatus(5i32);
#[repr(transparent)]
pub struct DISPID_SpeechBaseStream(pub i32);
pub const DISPID_SBSFormat: DISPID_SpeechBaseStream = DISPID_SpeechBaseStream(1i32);
pub const DISPID_SBSRead: DISPID_SpeechBaseStream = DISPID_SpeechBaseStream(2i32);
pub const DISPID_SBSWrite: DISPID_SpeechBaseStream = DISPID_SpeechBaseStream(3i32);
pub const DISPID_SBSSeek: DISPID_SpeechBaseStream = DISPID_SpeechBaseStream(4i32);
#[repr(transparent)]
pub struct DISPID_SpeechCustomStream(pub i32);
pub const DISPID_SCSBaseStream: DISPID_SpeechCustomStream = DISPID_SpeechCustomStream(100i32);
#[repr(transparent)]
pub struct DISPID_SpeechDataKey(pub i32);
pub const DISPID_SDKSetBinaryValue: DISPID_SpeechDataKey = DISPID_SpeechDataKey(1i32);
pub const DISPID_SDKGetBinaryValue: DISPID_SpeechDataKey = DISPID_SpeechDataKey(2i32);
pub const DISPID_SDKSetStringValue: DISPID_SpeechDataKey = DISPID_SpeechDataKey(3i32);
pub const DISPID_SDKGetStringValue: DISPID_SpeechDataKey = DISPID_SpeechDataKey(4i32);
pub const DISPID_SDKSetLongValue: DISPID_SpeechDataKey = DISPID_SpeechDataKey(5i32);
pub const DISPID_SDKGetlongValue: DISPID_SpeechDataKey = DISPID_SpeechDataKey(6i32);
pub const DISPID_SDKOpenKey: DISPID_SpeechDataKey = DISPID_SpeechDataKey(7i32);
pub const DISPID_SDKCreateKey: DISPID_SpeechDataKey = DISPID_SpeechDataKey(8i32);
pub const DISPID_SDKDeleteKey: DISPID_SpeechDataKey = DISPID_SpeechDataKey(9i32);
pub const DISPID_SDKDeleteValue: DISPID_SpeechDataKey = DISPID_SpeechDataKey(10i32);
pub const DISPID_SDKEnumKeys: DISPID_SpeechDataKey = DISPID_SpeechDataKey(11i32);
pub const DISPID_SDKEnumValues: DISPID_SpeechDataKey = DISPID_SpeechDataKey(12i32);
#[repr(transparent)]
pub struct DISPID_SpeechFileStream(pub i32);
pub const DISPID_SFSOpen: DISPID_SpeechFileStream = DISPID_SpeechFileStream(100i32);
pub const DISPID_SFSClose: DISPID_SpeechFileStream = DISPID_SpeechFileStream(101i32);
#[repr(transparent)]
pub struct DISPID_SpeechGrammarRule(pub i32);
pub const DISPID_SGRAttributes: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(1i32);
pub const DISPID_SGRInitialState: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(2i32);
pub const DISPID_SGRName: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(3i32);
pub const DISPID_SGRId: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(4i32);
pub const DISPID_SGRClear: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(5i32);
pub const DISPID_SGRAddResource: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(6i32);
pub const DISPID_SGRAddState: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(7i32);
#[repr(transparent)]
pub struct DISPID_SpeechGrammarRuleState(pub i32);
pub const DISPID_SGRSRule: DISPID_SpeechGrammarRuleState = DISPID_SpeechGrammarRuleState(1i32);
pub const DISPID_SGRSTransitions: DISPID_SpeechGrammarRuleState = DISPID_SpeechGrammarRuleState(2i32);
pub const DISPID_SGRSAddWordTransition: DISPID_SpeechGrammarRuleState = DISPID_SpeechGrammarRuleState(3i32);
pub const DISPID_SGRSAddRuleTransition: DISPID_SpeechGrammarRuleState = DISPID_SpeechGrammarRuleState(4i32);
pub const DISPID_SGRSAddSpecialTransition: DISPID_SpeechGrammarRuleState = DISPID_SpeechGrammarRuleState(5i32);
#[repr(transparent)]
pub struct DISPID_SpeechGrammarRuleStateTransition(pub i32);
pub const DISPID_SGRSTType: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(1i32);
pub const DISPID_SGRSTText: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(2i32);
pub const DISPID_SGRSTRule: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(3i32);
pub const DISPID_SGRSTWeight: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(4i32);
pub const DISPID_SGRSTPropertyName: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(5i32);
pub const DISPID_SGRSTPropertyId: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(6i32);
pub const DISPID_SGRSTPropertyValue: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(7i32);
pub const DISPID_SGRSTNextState: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(8i32);
#[repr(transparent)]
pub struct DISPID_SpeechGrammarRuleStateTransitions(pub i32);
pub const DISPID_SGRSTsCount: DISPID_SpeechGrammarRuleStateTransitions = DISPID_SpeechGrammarRuleStateTransitions(1i32);
pub const DISPID_SGRSTsItem: DISPID_SpeechGrammarRuleStateTransitions = DISPID_SpeechGrammarRuleStateTransitions(0i32);
pub const DISPID_SGRSTs_NewEnum: DISPID_SpeechGrammarRuleStateTransitions = DISPID_SpeechGrammarRuleStateTransitions(-4i32);
#[repr(transparent)]
pub struct DISPID_SpeechGrammarRules(pub i32);
pub const DISPID_SGRsCount: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(1i32);
pub const DISPID_SGRsDynamic: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(2i32);
pub const DISPID_SGRsAdd: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(3i32);
pub const DISPID_SGRsCommit: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(4i32);
pub const DISPID_SGRsCommitAndSave: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(5i32);
pub const DISPID_SGRsFindRule: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(6i32);
pub const DISPID_SGRsItem: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(0i32);
pub const DISPID_SGRs_NewEnum: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(-4i32);
#[repr(transparent)]
pub struct DISPID_SpeechLexicon(pub i32);
pub const DISPID_SLGenerationId: DISPID_SpeechLexicon = DISPID_SpeechLexicon(1i32);
pub const DISPID_SLGetWords: DISPID_SpeechLexicon = DISPID_SpeechLexicon(2i32);
pub const DISPID_SLAddPronunciation: DISPID_SpeechLexicon = DISPID_SpeechLexicon(3i32);
pub const DISPID_SLAddPronunciationByPhoneIds: DISPID_SpeechLexicon = DISPID_SpeechLexicon(4i32);
pub const DISPID_SLRemovePronunciation: DISPID_SpeechLexicon = DISPID_SpeechLexicon(5i32);
pub const DISPID_SLRemovePronunciationByPhoneIds: DISPID_SpeechLexicon = DISPID_SpeechLexicon(6i32);
pub const DISPID_SLGetPronunciations: DISPID_SpeechLexicon = DISPID_SpeechLexicon(7i32);
pub const DISPID_SLGetGenerationChange: DISPID_SpeechLexicon = DISPID_SpeechLexicon(8i32);
#[repr(transparent)]
pub struct DISPID_SpeechLexiconProns(pub i32);
pub const DISPID_SLPsCount: DISPID_SpeechLexiconProns = DISPID_SpeechLexiconProns(1i32);
pub const DISPID_SLPsItem: DISPID_SpeechLexiconProns = DISPID_SpeechLexiconProns(0i32);
pub const DISPID_SLPs_NewEnum: DISPID_SpeechLexiconProns = DISPID_SpeechLexiconProns(-4i32);
#[repr(transparent)]
pub struct DISPID_SpeechLexiconPronunciation(pub i32);
pub const DISPID_SLPType: DISPID_SpeechLexiconPronunciation = DISPID_SpeechLexiconPronunciation(1i32);
pub const DISPID_SLPLangId: DISPID_SpeechLexiconPronunciation = DISPID_SpeechLexiconPronunciation(2i32);
pub const DISPID_SLPPartOfSpeech: DISPID_SpeechLexiconPronunciation = DISPID_SpeechLexiconPronunciation(3i32);
pub const DISPID_SLPPhoneIds: DISPID_SpeechLexiconPronunciation = DISPID_SpeechLexiconPronunciation(4i32);
pub const DISPID_SLPSymbolic: DISPID_SpeechLexiconPronunciation = DISPID_SpeechLexiconPronunciation(5i32);
#[repr(transparent)]
pub struct DISPID_SpeechLexiconWord(pub i32);
pub const DISPID_SLWLangId: DISPID_SpeechLexiconWord = DISPID_SpeechLexiconWord(1i32);
pub const DISPID_SLWType: DISPID_SpeechLexiconWord = DISPID_SpeechLexiconWord(2i32);
pub const DISPID_SLWWord: DISPID_SpeechLexiconWord = DISPID_SpeechLexiconWord(3i32);
pub const DISPID_SLWPronunciations: DISPID_SpeechLexiconWord = DISPID_SpeechLexiconWord(4i32);
#[repr(transparent)]
pub struct DISPID_SpeechLexiconWords(pub i32);
pub const DISPID_SLWsCount: DISPID_SpeechLexiconWords = DISPID_SpeechLexiconWords(1i32);
pub const DISPID_SLWsItem: DISPID_SpeechLexiconWords = DISPID_SpeechLexiconWords(0i32);
pub const DISPID_SLWs_NewEnum: DISPID_SpeechLexiconWords = DISPID_SpeechLexiconWords(-4i32);
#[repr(transparent)]
pub struct DISPID_SpeechMMSysAudio(pub i32);
pub const DISPID_SMSADeviceId: DISPID_SpeechMMSysAudio = DISPID_SpeechMMSysAudio(300i32);
pub const DISPID_SMSALineId: DISPID_SpeechMMSysAudio = DISPID_SpeechMMSysAudio(301i32);
pub const DISPID_SMSAMMHandle: DISPID_SpeechMMSysAudio = DISPID_SpeechMMSysAudio(302i32);
#[repr(transparent)]
pub struct DISPID_SpeechMemoryStream(pub i32);
pub const DISPID_SMSSetData: DISPID_SpeechMemoryStream = DISPID_SpeechMemoryStream(100i32);
pub const DISPID_SMSGetData: DISPID_SpeechMemoryStream = DISPID_SpeechMemoryStream(101i32);
#[repr(transparent)]
pub struct DISPID_SpeechObjectToken(pub i32);
pub const DISPID_SOTId: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(1i32);
pub const DISPID_SOTDataKey: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(2i32);
pub const DISPID_SOTCategory: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(3i32);
pub const DISPID_SOTGetDescription: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(4i32);
pub const DISPID_SOTSetId: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(5i32);
pub const DISPID_SOTGetAttribute: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(6i32);
pub const DISPID_SOTCreateInstance: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(7i32);
pub const DISPID_SOTRemove: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(8i32);
pub const DISPID_SOTGetStorageFileName: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(9i32);
pub const DISPID_SOTRemoveStorageFileName: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(10i32);
pub const DISPID_SOTIsUISupported: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(11i32);
pub const DISPID_SOTDisplayUI: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(12i32);
pub const DISPID_SOTMatchesAttributes: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(13i32);
#[repr(transparent)]
pub struct DISPID_SpeechObjectTokenCategory(pub i32);
pub const DISPID_SOTCId: DISPID_SpeechObjectTokenCategory = DISPID_SpeechObjectTokenCategory(1i32);
pub const DISPID_SOTCDefault: DISPID_SpeechObjectTokenCategory = DISPID_SpeechObjectTokenCategory(2i32);
pub const DISPID_SOTCSetId: DISPID_SpeechObjectTokenCategory = DISPID_SpeechObjectTokenCategory(3i32);
pub const DISPID_SOTCGetDataKey: DISPID_SpeechObjectTokenCategory = DISPID_SpeechObjectTokenCategory(4i32);
pub const DISPID_SOTCEnumerateTokens: DISPID_SpeechObjectTokenCategory = DISPID_SpeechObjectTokenCategory(5i32);
#[repr(transparent)]
pub struct DISPID_SpeechObjectTokens(pub i32);
pub const DISPID_SOTsCount: DISPID_SpeechObjectTokens = DISPID_SpeechObjectTokens(1i32);
pub const DISPID_SOTsItem: DISPID_SpeechObjectTokens = DISPID_SpeechObjectTokens(0i32);
pub const DISPID_SOTs_NewEnum: DISPID_SpeechObjectTokens = DISPID_SpeechObjectTokens(-4i32);
#[repr(transparent)]
pub struct DISPID_SpeechPhoneConverter(pub i32);
pub const DISPID_SPCLangId: DISPID_SpeechPhoneConverter = DISPID_SpeechPhoneConverter(1i32);
pub const DISPID_SPCPhoneToId: DISPID_SpeechPhoneConverter = DISPID_SpeechPhoneConverter(2i32);
pub const DISPID_SPCIdToPhone: DISPID_SpeechPhoneConverter = DISPID_SpeechPhoneConverter(3i32);
#[repr(transparent)]
pub struct DISPID_SpeechPhraseAlternate(pub i32);
pub const DISPID_SPARecoResult: DISPID_SpeechPhraseAlternate = DISPID_SpeechPhraseAlternate(1i32);
pub const DISPID_SPAStartElementInResult: DISPID_SpeechPhraseAlternate = DISPID_SpeechPhraseAlternate(2i32);
pub const DISPID_SPANumberOfElementsInResult: DISPID_SpeechPhraseAlternate = DISPID_SpeechPhraseAlternate(3i32);
pub const DISPID_SPAPhraseInfo: DISPID_SpeechPhraseAlternate = DISPID_SpeechPhraseAlternate(4i32);
pub const DISPID_SPACommit: DISPID_SpeechPhraseAlternate = DISPID_SpeechPhraseAlternate(5i32);
#[repr(transparent)]
pub struct DISPID_SpeechPhraseAlternates(pub i32);
pub const DISPID_SPAsCount: DISPID_SpeechPhraseAlternates = DISPID_SpeechPhraseAlternates(1i32);
pub const DISPID_SPAsItem: DISPID_SpeechPhraseAlternates = DISPID_SpeechPhraseAlternates(0i32);
pub const DISPID_SPAs_NewEnum: DISPID_SpeechPhraseAlternates = DISPID_SpeechPhraseAlternates(-4i32);
#[repr(transparent)]
pub struct DISPID_SpeechPhraseBuilder(pub i32);
pub const DISPID_SPPBRestorePhraseFromMemory: DISPID_SpeechPhraseBuilder = DISPID_SpeechPhraseBuilder(1i32);
#[repr(transparent)]
pub struct DISPID_SpeechPhraseElement(pub i32);
pub const DISPID_SPEAudioTimeOffset: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(1i32);
pub const DISPID_SPEAudioSizeTime: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(2i32);
pub const DISPID_SPEAudioStreamOffset: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(3i32);
pub const DISPID_SPEAudioSizeBytes: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(4i32);
pub const DISPID_SPERetainedStreamOffset: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(5i32);
pub const DISPID_SPERetainedSizeBytes: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(6i32);
pub const DISPID_SPEDisplayText: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(7i32);
pub const DISPID_SPELexicalForm: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(8i32);
pub const DISPID_SPEPronunciation: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(9i32);
pub const DISPID_SPEDisplayAttributes: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(10i32);
pub const DISPID_SPERequiredConfidence: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(11i32);
pub const DISPID_SPEActualConfidence: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(12i32);
pub const DISPID_SPEEngineConfidence: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(13i32);
#[repr(transparent)]
pub struct DISPID_SpeechPhraseElements(pub i32);
pub const DISPID_SPEsCount: DISPID_SpeechPhraseElements = DISPID_SpeechPhraseElements(1i32);
pub const DISPID_SPEsItem: DISPID_SpeechPhraseElements = DISPID_SpeechPhraseElements(0i32);
pub const DISPID_SPEs_NewEnum: DISPID_SpeechPhraseElements = DISPID_SpeechPhraseElements(-4i32);
#[repr(transparent)]
pub struct DISPID_SpeechPhraseInfo(pub i32);
pub const DISPID_SPILanguageId: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(1i32);
pub const DISPID_SPIGrammarId: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(2i32);
pub const DISPID_SPIStartTime: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(3i32);
pub const DISPID_SPIAudioStreamPosition: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(4i32);
pub const DISPID_SPIAudioSizeBytes: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(5i32);
pub const DISPID_SPIRetainedSizeBytes: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(6i32);
pub const DISPID_SPIAudioSizeTime: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(7i32);
pub const DISPID_SPIRule: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(8i32);
pub const DISPID_SPIProperties: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(9i32);
pub const DISPID_SPIElements: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(10i32);
pub const DISPID_SPIReplacements: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(11i32);
pub const DISPID_SPIEngineId: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(12i32);
pub const DISPID_SPIEnginePrivateData: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(13i32);
pub const DISPID_SPISaveToMemory: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(14i32);
pub const DISPID_SPIGetText: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(15i32);
pub const DISPID_SPIGetDisplayAttributes: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(16i32);
#[repr(transparent)]
pub struct DISPID_SpeechPhraseProperties(pub i32);
pub const DISPID_SPPsCount: DISPID_SpeechPhraseProperties = DISPID_SpeechPhraseProperties(1i32);
pub const DISPID_SPPsItem: DISPID_SpeechPhraseProperties = DISPID_SpeechPhraseProperties(0i32);
pub const DISPID_SPPs_NewEnum: DISPID_SpeechPhraseProperties = DISPID_SpeechPhraseProperties(-4i32);
#[repr(transparent)]
pub struct DISPID_SpeechPhraseProperty(pub i32);
pub const DISPID_SPPName: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(1i32);
pub const DISPID_SPPId: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(2i32);
pub const DISPID_SPPValue: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(3i32);
pub const DISPID_SPPFirstElement: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(4i32);
pub const DISPID_SPPNumberOfElements: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(5i32);
pub const DISPID_SPPEngineConfidence: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(6i32);
pub const DISPID_SPPConfidence: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(7i32);
pub const DISPID_SPPParent: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(8i32);
pub const DISPID_SPPChildren: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(9i32);
#[repr(transparent)]
pub struct DISPID_SpeechPhraseReplacement(pub i32);
pub const DISPID_SPRDisplayAttributes: DISPID_SpeechPhraseReplacement = DISPID_SpeechPhraseReplacement(1i32);
pub const DISPID_SPRText: DISPID_SpeechPhraseReplacement = DISPID_SpeechPhraseReplacement(2i32);
pub const DISPID_SPRFirstElement: DISPID_SpeechPhraseReplacement = DISPID_SpeechPhraseReplacement(3i32);
pub const DISPID_SPRNumberOfElements: DISPID_SpeechPhraseReplacement = DISPID_SpeechPhraseReplacement(4i32);
#[repr(transparent)]
pub struct DISPID_SpeechPhraseReplacements(pub i32);
pub const DISPID_SPRsCount: DISPID_SpeechPhraseReplacements = DISPID_SpeechPhraseReplacements(1i32);
pub const DISPID_SPRsItem: DISPID_SpeechPhraseReplacements = DISPID_SpeechPhraseReplacements(0i32);
pub const DISPID_SPRs_NewEnum: DISPID_SpeechPhraseReplacements = DISPID_SpeechPhraseReplacements(-4i32);
#[repr(transparent)]
pub struct DISPID_SpeechPhraseRule(pub i32);
pub const DISPID_SPRuleName: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(1i32);
pub const DISPID_SPRuleId: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(2i32);
pub const DISPID_SPRuleFirstElement: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(3i32);
pub const DISPID_SPRuleNumberOfElements: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(4i32);
pub const DISPID_SPRuleParent: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(5i32);
pub const DISPID_SPRuleChildren: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(6i32);
pub const DISPID_SPRuleConfidence: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(7i32);
pub const DISPID_SPRuleEngineConfidence: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(8i32);
#[repr(transparent)]
pub struct DISPID_SpeechPhraseRules(pub i32);
pub const DISPID_SPRulesCount: DISPID_SpeechPhraseRules = DISPID_SpeechPhraseRules(1i32);
pub const DISPID_SPRulesItem: DISPID_SpeechPhraseRules = DISPID_SpeechPhraseRules(0i32);
pub const DISPID_SPRules_NewEnum: DISPID_SpeechPhraseRules = DISPID_SpeechPhraseRules(-4i32);
#[repr(transparent)]
pub struct DISPID_SpeechRecoContext(pub i32);
pub const DISPID_SRCRecognizer: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(1i32);
pub const DISPID_SRCAudioInInterferenceStatus: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(2i32);
pub const DISPID_SRCRequestedUIType: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(3i32);
pub const DISPID_SRCVoice: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(4i32);
pub const DISPID_SRAllowVoiceFormatMatchingOnNextSet: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(5i32);
pub const DISPID_SRCVoicePurgeEvent: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(6i32);
pub const DISPID_SRCEventInterests: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(7i32);
pub const DISPID_SRCCmdMaxAlternates: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(8i32);
pub const DISPID_SRCState: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(9i32);
pub const DISPID_SRCRetainedAudio: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(10i32);
pub const DISPID_SRCRetainedAudioFormat: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(11i32);
pub const DISPID_SRCPause: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(12i32);
pub const DISPID_SRCResume: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(13i32);
pub const DISPID_SRCCreateGrammar: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(14i32);
pub const DISPID_SRCCreateResultFromMemory: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(15i32);
pub const DISPID_SRCBookmark: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(16i32);
pub const DISPID_SRCSetAdaptationData: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(17i32);
#[repr(transparent)]
pub struct DISPID_SpeechRecoContextEvents(pub i32);
pub const DISPID_SRCEStartStream: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(1i32);
pub const DISPID_SRCEEndStream: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(2i32);
pub const DISPID_SRCEBookmark: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(3i32);
pub const DISPID_SRCESoundStart: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(4i32);
pub const DISPID_SRCESoundEnd: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(5i32);
pub const DISPID_SRCEPhraseStart: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(6i32);
pub const DISPID_SRCERecognition: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(7i32);
pub const DISPID_SRCEHypothesis: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(8i32);
pub const DISPID_SRCEPropertyNumberChange: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(9i32);
pub const DISPID_SRCEPropertyStringChange: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(10i32);
pub const DISPID_SRCEFalseRecognition: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(11i32);
pub const DISPID_SRCEInterference: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(12i32);
pub const DISPID_SRCERequestUI: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(13i32);
pub const DISPID_SRCERecognizerStateChange: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(14i32);
pub const DISPID_SRCEAdaptation: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(15i32);
pub const DISPID_SRCERecognitionForOtherContext: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(16i32);
pub const DISPID_SRCEAudioLevel: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(17i32);
pub const DISPID_SRCEEnginePrivate: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(18i32);
#[repr(transparent)]
pub struct DISPID_SpeechRecoResult(pub i32);
pub const DISPID_SRRRecoContext: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(1i32);
pub const DISPID_SRRTimes: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(2i32);
pub const DISPID_SRRAudioFormat: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(3i32);
pub const DISPID_SRRPhraseInfo: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(4i32);
pub const DISPID_SRRAlternates: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(5i32);
pub const DISPID_SRRAudio: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(6i32);
pub const DISPID_SRRSpeakAudio: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(7i32);
pub const DISPID_SRRSaveToMemory: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(8i32);
pub const DISPID_SRRDiscardResultInfo: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(9i32);
#[repr(transparent)]
pub struct DISPID_SpeechRecoResult2(pub i32);
pub const DISPID_SRRSetTextFeedback: DISPID_SpeechRecoResult2 = DISPID_SpeechRecoResult2(12i32);
#[repr(transparent)]
pub struct DISPID_SpeechRecoResultTimes(pub i32);
pub const DISPID_SRRTStreamTime: DISPID_SpeechRecoResultTimes = DISPID_SpeechRecoResultTimes(1i32);
pub const DISPID_SRRTLength: DISPID_SpeechRecoResultTimes = DISPID_SpeechRecoResultTimes(2i32);
pub const DISPID_SRRTTickCount: DISPID_SpeechRecoResultTimes = DISPID_SpeechRecoResultTimes(3i32);
pub const DISPID_SRRTOffsetFromStart: DISPID_SpeechRecoResultTimes = DISPID_SpeechRecoResultTimes(4i32);
#[repr(transparent)]
pub struct DISPID_SpeechRecognizer(pub i32);
pub const DISPID_SRRecognizer: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(1i32);
pub const DISPID_SRAllowAudioInputFormatChangesOnNextSet: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(2i32);
pub const DISPID_SRAudioInput: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(3i32);
pub const DISPID_SRAudioInputStream: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(4i32);
pub const DISPID_SRIsShared: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(5i32);
pub const DISPID_SRState: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(6i32);
pub const DISPID_SRStatus: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(7i32);
pub const DISPID_SRProfile: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(8i32);
pub const DISPID_SREmulateRecognition: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(9i32);
pub const DISPID_SRCreateRecoContext: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(10i32);
pub const DISPID_SRGetFormat: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(11i32);
pub const DISPID_SRSetPropertyNumber: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(12i32);
pub const DISPID_SRGetPropertyNumber: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(13i32);
pub const DISPID_SRSetPropertyString: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(14i32);
pub const DISPID_SRGetPropertyString: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(15i32);
pub const DISPID_SRIsUISupported: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(16i32);
pub const DISPID_SRDisplayUI: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(17i32);
pub const DISPID_SRGetRecognizers: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(18i32);
pub const DISPID_SVGetAudioInputs: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(19i32);
pub const DISPID_SVGetProfiles: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(20i32);
#[repr(transparent)]
pub struct DISPID_SpeechRecognizerStatus(pub i32);
pub const DISPID_SRSAudioStatus: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(1i32);
pub const DISPID_SRSCurrentStreamPosition: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(2i32);
pub const DISPID_SRSCurrentStreamNumber: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(3i32);
pub const DISPID_SRSNumberOfActiveRules: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(4i32);
pub const DISPID_SRSClsidEngine: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(5i32);
pub const DISPID_SRSSupportedLanguages: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(6i32);
#[repr(transparent)]
pub struct DISPID_SpeechVoice(pub i32);
pub const DISPID_SVStatus: DISPID_SpeechVoice = DISPID_SpeechVoice(1i32);
pub const DISPID_SVVoice: DISPID_SpeechVoice = DISPID_SpeechVoice(2i32);
pub const DISPID_SVAudioOutput: DISPID_SpeechVoice = DISPID_SpeechVoice(3i32);
pub const DISPID_SVAudioOutputStream: DISPID_SpeechVoice = DISPID_SpeechVoice(4i32);
pub const DISPID_SVRate: DISPID_SpeechVoice = DISPID_SpeechVoice(5i32);
pub const DISPID_SVVolume: DISPID_SpeechVoice = DISPID_SpeechVoice(6i32);
pub const DISPID_SVAllowAudioOuputFormatChangesOnNextSet: DISPID_SpeechVoice = DISPID_SpeechVoice(7i32);
pub const DISPID_SVEventInterests: DISPID_SpeechVoice = DISPID_SpeechVoice(8i32);
pub const DISPID_SVPriority: DISPID_SpeechVoice = DISPID_SpeechVoice(9i32);
pub const DISPID_SVAlertBoundary: DISPID_SpeechVoice = DISPID_SpeechVoice(10i32);
pub const DISPID_SVSyncronousSpeakTimeout: DISPID_SpeechVoice = DISPID_SpeechVoice(11i32);
pub const DISPID_SVSpeak: DISPID_SpeechVoice = DISPID_SpeechVoice(12i32);
pub const DISPID_SVSpeakStream: DISPID_SpeechVoice = DISPID_SpeechVoice(13i32);
pub const DISPID_SVPause: DISPID_SpeechVoice = DISPID_SpeechVoice(14i32);
pub const DISPID_SVResume: DISPID_SpeechVoice = DISPID_SpeechVoice(15i32);
pub const DISPID_SVSkip: DISPID_SpeechVoice = DISPID_SpeechVoice(16i32);
pub const DISPID_SVGetVoices: DISPID_SpeechVoice = DISPID_SpeechVoice(17i32);
pub const DISPID_SVGetAudioOutputs: DISPID_SpeechVoice = DISPID_SpeechVoice(18i32);
pub const DISPID_SVWaitUntilDone: DISPID_SpeechVoice = DISPID_SpeechVoice(19i32);
pub const DISPID_SVSpeakCompleteEvent: DISPID_SpeechVoice = DISPID_SpeechVoice(20i32);
pub const DISPID_SVIsUISupported: DISPID_SpeechVoice = DISPID_SpeechVoice(21i32);
pub const DISPID_SVDisplayUI: DISPID_SpeechVoice = DISPID_SpeechVoice(22i32);
#[repr(transparent)]
pub struct DISPID_SpeechVoiceEvent(pub i32);
pub const DISPID_SVEStreamStart: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(1i32);
pub const DISPID_SVEStreamEnd: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(2i32);
pub const DISPID_SVEVoiceChange: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(3i32);
pub const DISPID_SVEBookmark: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(4i32);
pub const DISPID_SVEWord: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(5i32);
pub const DISPID_SVEPhoneme: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(6i32);
pub const DISPID_SVESentenceBoundary: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(7i32);
pub const DISPID_SVEViseme: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(8i32);
pub const DISPID_SVEAudioLevel: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(9i32);
pub const DISPID_SVEEnginePrivate: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(10i32);
#[repr(transparent)]
pub struct DISPID_SpeechVoiceStatus(pub i32);
pub const DISPID_SVSCurrentStreamNumber: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(1i32);
pub const DISPID_SVSLastStreamNumberQueued: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(2i32);
pub const DISPID_SVSLastResult: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(3i32);
pub const DISPID_SVSRunningState: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(4i32);
pub const DISPID_SVSInputWordPosition: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(5i32);
pub const DISPID_SVSInputWordLength: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(6i32);
pub const DISPID_SVSInputSentencePosition: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(7i32);
pub const DISPID_SVSInputSentenceLength: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(8i32);
pub const DISPID_SVSLastBookmark: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(9i32);
pub const DISPID_SVSLastBookmarkId: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(10i32);
pub const DISPID_SVSPhonemeId: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(11i32);
pub const DISPID_SVSVisemeId: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(12i32);
#[repr(transparent)]
pub struct DISPID_SpeechWaveFormatEx(pub i32);
pub const DISPID_SWFEFormatTag: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(1i32);
pub const DISPID_SWFEChannels: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(2i32);
pub const DISPID_SWFESamplesPerSec: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(3i32);
pub const DISPID_SWFEAvgBytesPerSec: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(4i32);
pub const DISPID_SWFEBlockAlign: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(5i32);
pub const DISPID_SWFEBitsPerSample: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(6i32);
pub const DISPID_SWFEExtraData: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(7i32);
#[repr(transparent)]
pub struct DISPID_SpeechXMLRecoResult(pub i32);
pub const DISPID_SRRGetXMLResult: DISPID_SpeechXMLRecoResult = DISPID_SpeechXMLRecoResult(10i32);
pub const DISPID_SRRGetXMLErrorInfo: DISPID_SpeechXMLRecoResult = DISPID_SpeechXMLRecoResult(11i32);
#[repr(transparent)]
pub struct IEnumSpObjectTokens(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpAudio(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpContainerLexicon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpDataKey(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpDisplayAlternates(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpEnginePronunciation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpEventSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpEventSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpEventSource2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpGrammarBuilder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpGrammarBuilder2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpLexicon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpMMSysAudio(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpNotifyCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpNotifySink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpNotifySource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpNotifyTranslator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpObjectToken(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpObjectTokenCategory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpObjectTokenInit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpObjectWithToken(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpPhoneConverter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpPhoneticAlphabetConverter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpPhoneticAlphabetSelection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpPhrase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpPhrase2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpPhraseAlt(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpRecoContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpRecoContext2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpRecoGrammar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpRecoGrammar2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpRecoResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpRecoResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpRecognizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpRecognizer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpRegDataKey(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpResourceManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpSerializeState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpShortcut(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpStreamFormat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpStreamFormatConverter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpTranscript(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpVoice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpXMLRecoResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechAudio(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechAudioBufferInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechAudioFormat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechAudioStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechBaseStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechCustomStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechDataKey(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechFileStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechGrammarRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechGrammarRuleState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechGrammarRuleStateTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechGrammarRuleStateTransitions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechGrammarRules(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechLexicon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechLexiconPronunciation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechLexiconPronunciations(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechLexiconWord(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechLexiconWords(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechMMSysAudio(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechMemoryStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechObjectToken(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechObjectTokenCategory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechObjectTokens(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechPhoneConverter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechPhraseAlternate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechPhraseAlternates(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechPhraseElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechPhraseElements(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechPhraseInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechPhraseInfoBuilder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechPhraseProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechPhraseProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechPhraseReplacement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechPhraseReplacements(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechPhraseRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechPhraseRules(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecoContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecoGrammar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecoResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecoResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecoResultDispatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecoResultTimes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechRecognizerStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechResourceLoader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechTextSelectionInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechVoice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechVoiceStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechWaveFormatEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpeechXMLRecoResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PHONETICALPHABET(pub i32);
pub const PA_Ipa: PHONETICALPHABET = PHONETICALPHABET(0i32);
pub const PA_Ups: PHONETICALPHABET = PHONETICALPHABET(1i32);
pub const PA_Sapi: PHONETICALPHABET = PHONETICALPHABET(2i32);
pub const SAPI_ERROR_BASE: u32 = 20480u32;
#[repr(transparent)]
pub struct SPADAPTATIONRELEVANCE(pub i32);
pub const SPAR_Unknown: SPADAPTATIONRELEVANCE = SPADAPTATIONRELEVANCE(0i32);
pub const SPAR_Low: SPADAPTATIONRELEVANCE = SPADAPTATIONRELEVANCE(1i32);
pub const SPAR_Medium: SPADAPTATIONRELEVANCE = SPADAPTATIONRELEVANCE(2i32);
pub const SPAR_High: SPADAPTATIONRELEVANCE = SPADAPTATIONRELEVANCE(3i32);
#[repr(transparent)]
pub struct SPADAPTATIONSETTINGS(pub i32);
pub const SPADS_Default: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(0i32);
pub const SPADS_CurrentRecognizer: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(1i32);
pub const SPADS_RecoProfile: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(2i32);
pub const SPADS_Immediate: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(4i32);
pub const SPADS_Reset: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(8i32);
pub const SPADS_HighVolumeDataSource: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(16i32);
#[repr(C)]
pub struct SPAUDIOBUFFERINFO(i32);
#[repr(transparent)]
pub struct SPAUDIOOPTIONS(pub i32);
pub const SPAO_NONE: SPAUDIOOPTIONS = SPAUDIOOPTIONS(0i32);
pub const SPAO_RETAIN_AUDIO: SPAUDIOOPTIONS = SPAUDIOOPTIONS(1i32);
#[repr(transparent)]
pub struct SPAUDIOSTATE(pub i32);
pub const SPAS_CLOSED: SPAUDIOSTATE = SPAUDIOSTATE(0i32);
pub const SPAS_STOP: SPAUDIOSTATE = SPAUDIOSTATE(1i32);
pub const SPAS_PAUSE: SPAUDIOSTATE = SPAUDIOSTATE(2i32);
pub const SPAS_RUN: SPAUDIOSTATE = SPAUDIOSTATE(3i32);
#[repr(C)]
pub struct SPAUDIOSTATUS(i32);
#[repr(C)]
pub struct SPBINARYGRAMMAR(i32);
#[repr(transparent)]
pub struct SPBOOKMARKOPTIONS(pub i32);
pub const SPBO_NONE: SPBOOKMARKOPTIONS = SPBOOKMARKOPTIONS(0i32);
pub const SPBO_PAUSE: SPBOOKMARKOPTIONS = SPBOOKMARKOPTIONS(1i32);
pub const SPBO_AHEAD: SPBOOKMARKOPTIONS = SPBOOKMARKOPTIONS(2i32);
pub const SPBO_TIME_UNITS: SPBOOKMARKOPTIONS = SPBOOKMARKOPTIONS(4i32);
#[repr(transparent)]
pub struct SPCFGRULEATTRIBUTES(pub i32);
pub const SPRAF_TopLevel: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(1i32);
pub const SPRAF_Active: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(2i32);
pub const SPRAF_Export: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(4i32);
pub const SPRAF_Import: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(8i32);
pub const SPRAF_Interpreter: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(16i32);
pub const SPRAF_Dynamic: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(32i32);
pub const SPRAF_Root: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(64i32);
pub const SPRAF_AutoPause: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(65536i32);
pub const SPRAF_UserDelimited: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(131072i32);
#[repr(transparent)]
pub struct SPCOMMITFLAGS(pub i32);
pub const SPCF_NONE: SPCOMMITFLAGS = SPCOMMITFLAGS(0i32);
pub const SPCF_ADD_TO_USER_LEXICON: SPCOMMITFLAGS = SPCOMMITFLAGS(1i32);
pub const SPCF_DEFINITE_CORRECTION: SPCOMMITFLAGS = SPCOMMITFLAGS(2i32);
#[repr(transparent)]
pub struct SPCONTEXTSTATE(pub i32);
pub const SPCS_DISABLED: SPCONTEXTSTATE = SPCONTEXTSTATE(0i32);
pub const SPCS_ENABLED: SPCONTEXTSTATE = SPCONTEXTSTATE(1i32);
#[repr(transparent)]
pub struct SPDATAKEYLOCATION(pub i32);
pub const SPDKL_DefaultLocation: SPDATAKEYLOCATION = SPDATAKEYLOCATION(0i32);
pub const SPDKL_CurrentUser: SPDATAKEYLOCATION = SPDATAKEYLOCATION(1i32);
pub const SPDKL_LocalMachine: SPDATAKEYLOCATION = SPDATAKEYLOCATION(2i32);
pub const SPDKL_CurrentConfig: SPDATAKEYLOCATION = SPDATAKEYLOCATION(5i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SPDISPLAYPHRASE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SPDISPLAYTOKEN(i32);
#[repr(transparent)]
pub struct SPDISPLYATTRIBUTES(pub i32);
pub const SPAF_ONE_TRAILING_SPACE: SPDISPLYATTRIBUTES = SPDISPLYATTRIBUTES(2i32);
pub const SPAF_TWO_TRAILING_SPACES: SPDISPLYATTRIBUTES = SPDISPLYATTRIBUTES(4i32);
pub const SPAF_CONSUME_LEADING_SPACES: SPDISPLYATTRIBUTES = SPDISPLYATTRIBUTES(8i32);
pub const SPAF_BUFFER_POSITION: SPDISPLYATTRIBUTES = SPDISPLYATTRIBUTES(16i32);
pub const SPAF_ALL: SPDISPLYATTRIBUTES = SPDISPLYATTRIBUTES(31i32);
pub const SPAF_USER_SPECIFIED: SPDISPLYATTRIBUTES = SPDISPLYATTRIBUTES(128i32);
#[repr(transparent)]
pub struct SPEAKFLAGS(pub i32);
pub const SPF_DEFAULT: SPEAKFLAGS = SPEAKFLAGS(0i32);
pub const SPF_ASYNC: SPEAKFLAGS = SPEAKFLAGS(1i32);
pub const SPF_PURGEBEFORESPEAK: SPEAKFLAGS = SPEAKFLAGS(2i32);
pub const SPF_IS_FILENAME: SPEAKFLAGS = SPEAKFLAGS(4i32);
pub const SPF_IS_XML: SPEAKFLAGS = SPEAKFLAGS(8i32);
pub const SPF_IS_NOT_XML: SPEAKFLAGS = SPEAKFLAGS(16i32);
pub const SPF_PERSIST_XML: SPEAKFLAGS = SPEAKFLAGS(32i32);
pub const SPF_NLP_SPEAK_PUNC: SPEAKFLAGS = SPEAKFLAGS(64i32);
pub const SPF_PARSE_SAPI: SPEAKFLAGS = SPEAKFLAGS(128i32);
pub const SPF_PARSE_SSML: SPEAKFLAGS = SPEAKFLAGS(256i32);
pub const SPF_PARSE_AUTODETECT: SPEAKFLAGS = SPEAKFLAGS(0i32);
pub const SPF_NLP_MASK: SPEAKFLAGS = SPEAKFLAGS(64i32);
pub const SPF_PARSE_MASK: SPEAKFLAGS = SPEAKFLAGS(384i32);
pub const SPF_VOICE_MASK: SPEAKFLAGS = SPEAKFLAGS(511i32);
pub const SPF_UNUSED_FLAGS: SPEAKFLAGS = SPEAKFLAGS(-512i32);
#[repr(transparent)]
pub struct SPENDSRSTREAMFLAGS(pub i32);
pub const SPESF_NONE: SPENDSRSTREAMFLAGS = SPENDSRSTREAMFLAGS(0i32);
pub const SPESF_STREAM_RELEASED: SPENDSRSTREAMFLAGS = SPENDSRSTREAMFLAGS(1i32);
pub const SPESF_EMULATED: SPENDSRSTREAMFLAGS = SPENDSRSTREAMFLAGS(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SPEVENT(i32);
#[repr(transparent)]
pub struct SPEVENTENUM(pub i32);
pub const SPEI_UNDEFINED: SPEVENTENUM = SPEVENTENUM(0i32);
pub const SPEI_START_INPUT_STREAM: SPEVENTENUM = SPEVENTENUM(1i32);
pub const SPEI_END_INPUT_STREAM: SPEVENTENUM = SPEVENTENUM(2i32);
pub const SPEI_VOICE_CHANGE: SPEVENTENUM = SPEVENTENUM(3i32);
pub const SPEI_TTS_BOOKMARK: SPEVENTENUM = SPEVENTENUM(4i32);
pub const SPEI_WORD_BOUNDARY: SPEVENTENUM = SPEVENTENUM(5i32);
pub const SPEI_PHONEME: SPEVENTENUM = SPEVENTENUM(6i32);
pub const SPEI_SENTENCE_BOUNDARY: SPEVENTENUM = SPEVENTENUM(7i32);
pub const SPEI_VISEME: SPEVENTENUM = SPEVENTENUM(8i32);
pub const SPEI_TTS_AUDIO_LEVEL: SPEVENTENUM = SPEVENTENUM(9i32);
pub const SPEI_TTS_PRIVATE: SPEVENTENUM = SPEVENTENUM(15i32);
pub const SPEI_MIN_TTS: SPEVENTENUM = SPEVENTENUM(1i32);
pub const SPEI_MAX_TTS: SPEVENTENUM = SPEVENTENUM(15i32);
pub const SPEI_END_SR_STREAM: SPEVENTENUM = SPEVENTENUM(34i32);
pub const SPEI_SOUND_START: SPEVENTENUM = SPEVENTENUM(35i32);
pub const SPEI_SOUND_END: SPEVENTENUM = SPEVENTENUM(36i32);
pub const SPEI_PHRASE_START: SPEVENTENUM = SPEVENTENUM(37i32);
pub const SPEI_RECOGNITION: SPEVENTENUM = SPEVENTENUM(38i32);
pub const SPEI_HYPOTHESIS: SPEVENTENUM = SPEVENTENUM(39i32);
pub const SPEI_SR_BOOKMARK: SPEVENTENUM = SPEVENTENUM(40i32);
pub const SPEI_PROPERTY_NUM_CHANGE: SPEVENTENUM = SPEVENTENUM(41i32);
pub const SPEI_PROPERTY_STRING_CHANGE: SPEVENTENUM = SPEVENTENUM(42i32);
pub const SPEI_FALSE_RECOGNITION: SPEVENTENUM = SPEVENTENUM(43i32);
pub const SPEI_INTERFERENCE: SPEVENTENUM = SPEVENTENUM(44i32);
pub const SPEI_REQUEST_UI: SPEVENTENUM = SPEVENTENUM(45i32);
pub const SPEI_RECO_STATE_CHANGE: SPEVENTENUM = SPEVENTENUM(46i32);
pub const SPEI_ADAPTATION: SPEVENTENUM = SPEVENTENUM(47i32);
pub const SPEI_START_SR_STREAM: SPEVENTENUM = SPEVENTENUM(48i32);
pub const SPEI_RECO_OTHER_CONTEXT: SPEVENTENUM = SPEVENTENUM(49i32);
pub const SPEI_SR_AUDIO_LEVEL: SPEVENTENUM = SPEVENTENUM(50i32);
pub const SPEI_SR_RETAINEDAUDIO: SPEVENTENUM = SPEVENTENUM(51i32);
pub const SPEI_SR_PRIVATE: SPEVENTENUM = SPEVENTENUM(52i32);
pub const SPEI_RESERVED4: SPEVENTENUM = SPEVENTENUM(53i32);
pub const SPEI_RESERVED5: SPEVENTENUM = SPEVENTENUM(54i32);
pub const SPEI_RESERVED6: SPEVENTENUM = SPEVENTENUM(55i32);
pub const SPEI_MIN_SR: SPEVENTENUM = SPEVENTENUM(34i32);
pub const SPEI_MAX_SR: SPEVENTENUM = SPEVENTENUM(55i32);
pub const SPEI_RESERVED1: SPEVENTENUM = SPEVENTENUM(30i32);
pub const SPEI_RESERVED2: SPEVENTENUM = SPEVENTENUM(33i32);
pub const SPEI_RESERVED3: SPEVENTENUM = SPEVENTENUM(63i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SPEVENTEX(i32);
#[repr(transparent)]
pub struct SPEVENTLPARAMTYPE(pub i32);
pub const SPET_LPARAM_IS_UNDEFINED: SPEVENTLPARAMTYPE = SPEVENTLPARAMTYPE(0i32);
pub const SPET_LPARAM_IS_TOKEN: SPEVENTLPARAMTYPE = SPEVENTLPARAMTYPE(1i32);
pub const SPET_LPARAM_IS_OBJECT: SPEVENTLPARAMTYPE = SPEVENTLPARAMTYPE(2i32);
pub const SPET_LPARAM_IS_POINTER: SPEVENTLPARAMTYPE = SPEVENTLPARAMTYPE(3i32);
pub const SPET_LPARAM_IS_STRING: SPEVENTLPARAMTYPE = SPEVENTLPARAMTYPE(4i32);
#[repr(C)]
pub struct SPEVENTSOURCEINFO(i32);
#[repr(transparent)]
pub struct SPFILEMODE(pub i32);
pub const SPFM_OPEN_READONLY: SPFILEMODE = SPFILEMODE(0i32);
pub const SPFM_OPEN_READWRITE: SPFILEMODE = SPFILEMODE(1i32);
pub const SPFM_CREATE: SPFILEMODE = SPFILEMODE(2i32);
pub const SPFM_CREATE_ALWAYS: SPFILEMODE = SPFILEMODE(3i32);
pub const SPFM_NUM_MODES: SPFILEMODE = SPFILEMODE(4i32);
#[repr(transparent)]
pub struct SPGRAMMAROPTIONS(pub i32);
pub const SPGO_SAPI: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(1i32);
pub const SPGO_SRGS: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(2i32);
pub const SPGO_UPS: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(4i32);
pub const SPGO_SRGS_MS_SCRIPT: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(8i32);
pub const SPGO_SRGS_W3C_SCRIPT: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(256i32);
pub const SPGO_SRGS_STG_SCRIPT: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(512i32);
pub const SPGO_SRGS_SCRIPT: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(778i32);
pub const SPGO_FILE: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(16i32);
pub const SPGO_HTTP: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(32i32);
pub const SPGO_RES: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(64i32);
pub const SPGO_OBJECT: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(128i32);
pub const SPGO_DEFAULT: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(1019i32);
pub const SPGO_ALL: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(1023i32);
#[repr(transparent)]
pub struct SPGRAMMARSTATE(pub i32);
pub const SPGS_DISABLED: SPGRAMMARSTATE = SPGRAMMARSTATE(0i32);
pub const SPGS_ENABLED: SPGRAMMARSTATE = SPGRAMMARSTATE(1i32);
pub const SPGS_EXCLUSIVE: SPGRAMMARSTATE = SPGRAMMARSTATE(3i32);
#[repr(transparent)]
pub struct SPGRAMMARWORDTYPE(pub i32);
pub const SPWT_DISPLAY: SPGRAMMARWORDTYPE = SPGRAMMARWORDTYPE(0i32);
pub const SPWT_LEXICAL: SPGRAMMARWORDTYPE = SPGRAMMARWORDTYPE(1i32);
pub const SPWT_PRONUNCIATION: SPGRAMMARWORDTYPE = SPGRAMMARWORDTYPE(2i32);
pub const SPWT_LEXICAL_NO_SPECIAL_CHARS: SPGRAMMARWORDTYPE = SPGRAMMARWORDTYPE(3i32);
#[repr(transparent)]
pub struct SPINTERFERENCE(pub i32);
pub const SPINTERFERENCE_NONE: SPINTERFERENCE = SPINTERFERENCE(0i32);
pub const SPINTERFERENCE_NOISE: SPINTERFERENCE = SPINTERFERENCE(1i32);
pub const SPINTERFERENCE_NOSIGNAL: SPINTERFERENCE = SPINTERFERENCE(2i32);
pub const SPINTERFERENCE_TOOLOUD: SPINTERFERENCE = SPINTERFERENCE(3i32);
pub const SPINTERFERENCE_TOOQUIET: SPINTERFERENCE = SPINTERFERENCE(4i32);
pub const SPINTERFERENCE_TOOFAST: SPINTERFERENCE = SPINTERFERENCE(5i32);
pub const SPINTERFERENCE_TOOSLOW: SPINTERFERENCE = SPINTERFERENCE(6i32);
pub const SPINTERFERENCE_LATENCY_WARNING: SPINTERFERENCE = SPINTERFERENCE(7i32);
pub const SPINTERFERENCE_LATENCY_TRUNCATE_BEGIN: SPINTERFERENCE = SPINTERFERENCE(8i32);
pub const SPINTERFERENCE_LATENCY_TRUNCATE_END: SPINTERFERENCE = SPINTERFERENCE(9i32);
#[repr(transparent)]
pub struct SPLEXICONTYPE(pub i32);
pub const eLEXTYPE_USER: SPLEXICONTYPE = SPLEXICONTYPE(1i32);
pub const eLEXTYPE_APP: SPLEXICONTYPE = SPLEXICONTYPE(2i32);
pub const eLEXTYPE_VENDORLEXICON: SPLEXICONTYPE = SPLEXICONTYPE(4i32);
pub const eLEXTYPE_LETTERTOSOUND: SPLEXICONTYPE = SPLEXICONTYPE(8i32);
pub const eLEXTYPE_MORPHOLOGY: SPLEXICONTYPE = SPLEXICONTYPE(16i32);
pub const eLEXTYPE_RESERVED4: SPLEXICONTYPE = SPLEXICONTYPE(32i32);
pub const eLEXTYPE_USER_SHORTCUT: SPLEXICONTYPE = SPLEXICONTYPE(64i32);
pub const eLEXTYPE_RESERVED6: SPLEXICONTYPE = SPLEXICONTYPE(128i32);
pub const eLEXTYPE_RESERVED7: SPLEXICONTYPE = SPLEXICONTYPE(256i32);
pub const eLEXTYPE_RESERVED8: SPLEXICONTYPE = SPLEXICONTYPE(512i32);
pub const eLEXTYPE_RESERVED9: SPLEXICONTYPE = SPLEXICONTYPE(1024i32);
pub const eLEXTYPE_RESERVED10: SPLEXICONTYPE = SPLEXICONTYPE(2048i32);
pub const eLEXTYPE_PRIVATE1: SPLEXICONTYPE = SPLEXICONTYPE(4096i32);
pub const eLEXTYPE_PRIVATE2: SPLEXICONTYPE = SPLEXICONTYPE(8192i32);
pub const eLEXTYPE_PRIVATE3: SPLEXICONTYPE = SPLEXICONTYPE(16384i32);
pub const eLEXTYPE_PRIVATE4: SPLEXICONTYPE = SPLEXICONTYPE(32768i32);
pub const eLEXTYPE_PRIVATE5: SPLEXICONTYPE = SPLEXICONTYPE(65536i32);
pub const eLEXTYPE_PRIVATE6: SPLEXICONTYPE = SPLEXICONTYPE(131072i32);
pub const eLEXTYPE_PRIVATE7: SPLEXICONTYPE = SPLEXICONTYPE(262144i32);
pub const eLEXTYPE_PRIVATE8: SPLEXICONTYPE = SPLEXICONTYPE(524288i32);
pub const eLEXTYPE_PRIVATE9: SPLEXICONTYPE = SPLEXICONTYPE(1048576i32);
pub const eLEXTYPE_PRIVATE10: SPLEXICONTYPE = SPLEXICONTYPE(2097152i32);
pub const eLEXTYPE_PRIVATE11: SPLEXICONTYPE = SPLEXICONTYPE(4194304i32);
pub const eLEXTYPE_PRIVATE12: SPLEXICONTYPE = SPLEXICONTYPE(8388608i32);
pub const eLEXTYPE_PRIVATE13: SPLEXICONTYPE = SPLEXICONTYPE(16777216i32);
pub const eLEXTYPE_PRIVATE14: SPLEXICONTYPE = SPLEXICONTYPE(33554432i32);
pub const eLEXTYPE_PRIVATE15: SPLEXICONTYPE = SPLEXICONTYPE(67108864i32);
pub const eLEXTYPE_PRIVATE16: SPLEXICONTYPE = SPLEXICONTYPE(134217728i32);
pub const eLEXTYPE_PRIVATE17: SPLEXICONTYPE = SPLEXICONTYPE(268435456i32);
pub const eLEXTYPE_PRIVATE18: SPLEXICONTYPE = SPLEXICONTYPE(536870912i32);
pub const eLEXTYPE_PRIVATE19: SPLEXICONTYPE = SPLEXICONTYPE(1073741824i32);
pub const eLEXTYPE_PRIVATE20: SPLEXICONTYPE = SPLEXICONTYPE(-2147483648i32);
#[repr(transparent)]
pub struct SPLOADOPTIONS(pub i32);
pub const SPLO_STATIC: SPLOADOPTIONS = SPLOADOPTIONS(0i32);
pub const SPLO_DYNAMIC: SPLOADOPTIONS = SPLOADOPTIONS(1i32);
#[repr(transparent)]
pub struct SPMATCHINGMODE(pub i32);
pub const AllWords: SPMATCHINGMODE = SPMATCHINGMODE(0i32);
pub const Subsequence: SPMATCHINGMODE = SPMATCHINGMODE(1i32);
pub const OrderedSubset: SPMATCHINGMODE = SPMATCHINGMODE(3i32);
pub const SubsequenceContentRequired: SPMATCHINGMODE = SPMATCHINGMODE(5i32);
pub const OrderedSubsetContentRequired: SPMATCHINGMODE = SPMATCHINGMODE(7i32);
#[repr(C)]
pub struct SPNORMALIZATIONLIST(i32);
#[repr(C)]
pub struct SPNOTIFYCALLBACK(i32);
#[repr(transparent)]
pub struct SPPARTOFSPEECH(pub i32);
pub const SPPS_NotOverriden: SPPARTOFSPEECH = SPPARTOFSPEECH(-1i32);
pub const SPPS_Unknown: SPPARTOFSPEECH = SPPARTOFSPEECH(0i32);
pub const SPPS_Noun: SPPARTOFSPEECH = SPPARTOFSPEECH(4096i32);
pub const SPPS_Verb: SPPARTOFSPEECH = SPPARTOFSPEECH(8192i32);
pub const SPPS_Modifier: SPPARTOFSPEECH = SPPARTOFSPEECH(12288i32);
pub const SPPS_Function: SPPARTOFSPEECH = SPPARTOFSPEECH(16384i32);
pub const SPPS_Interjection: SPPARTOFSPEECH = SPPARTOFSPEECH(20480i32);
pub const SPPS_Noncontent: SPPARTOFSPEECH = SPPARTOFSPEECH(24576i32);
pub const SPPS_LMA: SPPARTOFSPEECH = SPPARTOFSPEECH(28672i32);
pub const SPPS_SuppressWord: SPPARTOFSPEECH = SPPARTOFSPEECH(61440i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct SPPHRASE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SPPHRASEELEMENT(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct SPPHRASEPROPERTY(i32);
#[repr(transparent)]
pub struct SPPHRASEPROPERTYUNIONTYPE(pub i32);
pub const SPPPUT_UNUSED: SPPHRASEPROPERTYUNIONTYPE = SPPHRASEPROPERTYUNIONTYPE(0i32);
pub const SPPPUT_ARRAY_INDEX: SPPHRASEPROPERTYUNIONTYPE = SPPHRASEPROPERTYUNIONTYPE(1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SPPHRASEREPLACEMENT(i32);
#[repr(transparent)]
pub struct SPPHRASERNG(pub i32);
pub const SPPR_ALL_ELEMENTS: SPPHRASERNG = SPPHRASERNG(-1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SPPHRASERULE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct SPPHRASE_50(i32);
#[repr(transparent)]
pub struct SPPRONUNCIATIONFLAGS(pub i32);
pub const ePRONFLAG_USED: SPPRONUNCIATIONFLAGS = SPPRONUNCIATIONFLAGS(1i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct SPPROPERTYINFO(i32);
#[repr(C)]
pub struct SPRECOCONTEXTSTATUS(i32);
#[repr(transparent)]
pub struct SPRECOEVENTFLAGS(pub i32);
pub const SPREF_AutoPause: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(1i32);
pub const SPREF_Emulated: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(2i32);
pub const SPREF_SMLTimeout: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(4i32);
pub const SPREF_ExtendableParse: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(8i32);
pub const SPREF_ReSent: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(16i32);
pub const SPREF_Hypothesis: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(32i32);
pub const SPREF_FalseRecognition: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(64i32);
#[repr(C)]
pub struct SPRECOGNIZERSTATUS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SPRECORESULTTIMES(i32);
#[repr(transparent)]
pub struct SPRECOSTATE(pub i32);
pub const SPRST_INACTIVE: SPRECOSTATE = SPRECOSTATE(0i32);
pub const SPRST_ACTIVE: SPRECOSTATE = SPRECOSTATE(1i32);
pub const SPRST_ACTIVE_ALWAYS: SPRECOSTATE = SPRECOSTATE(2i32);
pub const SPRST_INACTIVE_WITH_PURGE: SPRECOSTATE = SPRECOSTATE(3i32);
pub const SPRST_NUM_STATES: SPRECOSTATE = SPRECOSTATE(4i32);
pub const SPRP_NORMAL: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SPRULE(i32);
#[repr(transparent)]
pub struct SPRULESTATE(pub i32);
pub const SPRS_INACTIVE: SPRULESTATE = SPRULESTATE(0i32);
pub const SPRS_ACTIVE: SPRULESTATE = SPRULESTATE(1i32);
pub const SPRS_ACTIVE_WITH_AUTO_PAUSE: SPRULESTATE = SPRULESTATE(3i32);
pub const SPRS_ACTIVE_USER_DELIMITED: SPRULESTATE = SPRULESTATE(4i32);
#[repr(transparent)]
pub struct SPRUNSTATE(pub i32);
pub const SPRS_DONE: SPRUNSTATE = SPRUNSTATE(1i32);
pub const SPRS_IS_SPEAKING: SPRUNSTATE = SPRUNSTATE(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SPSEMANTICERRORINFO(i32);
#[repr(transparent)]
pub struct SPSEMANTICFORMAT(pub i32);
pub const SPSMF_SAPI_PROPERTIES: SPSEMANTICFORMAT = SPSEMANTICFORMAT(0i32);
pub const SPSMF_SRGS_SEMANTICINTERPRETATION_MS: SPSEMANTICFORMAT = SPSEMANTICFORMAT(1i32);
pub const SPSMF_SRGS_SAPIPROPERTIES: SPSEMANTICFORMAT = SPSEMANTICFORMAT(2i32);
pub const SPSMF_UPS: SPSEMANTICFORMAT = SPSEMANTICFORMAT(4i32);
pub const SPSMF_SRGS_SEMANTICINTERPRETATION_W3C: SPSEMANTICFORMAT = SPSEMANTICFORMAT(8i32);
#[repr(C)]
pub struct SPSERIALIZEDEVENT(i32);
#[repr(C)]
pub struct SPSERIALIZEDEVENT64(i32);
#[repr(C)]
pub struct SPSERIALIZEDPHRASE(i32);
#[repr(C)]
pub struct SPSERIALIZEDRESULT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SPSHORTCUTPAIR(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SPSHORTCUTPAIRLIST(i32);
#[repr(transparent)]
pub struct SPSHORTCUTTYPE(pub i32);
pub const SPSHT_NotOverriden: SPSHORTCUTTYPE = SPSHORTCUTTYPE(-1i32);
pub const SPSHT_Unknown: SPSHORTCUTTYPE = SPSHORTCUTTYPE(0i32);
pub const SPSHT_EMAIL: SPSHORTCUTTYPE = SPSHORTCUTTYPE(4096i32);
pub const SPSHT_OTHER: SPSHORTCUTTYPE = SPSHORTCUTTYPE(8192i32);
pub const SPPS_RESERVED1: SPSHORTCUTTYPE = SPSHORTCUTTYPE(12288i32);
pub const SPPS_RESERVED2: SPSHORTCUTTYPE = SPSHORTCUTTYPE(16384i32);
pub const SPPS_RESERVED3: SPSHORTCUTTYPE = SPSHORTCUTTYPE(20480i32);
pub const SPPS_RESERVED4: SPSHORTCUTTYPE = SPSHORTCUTTYPE(61440i32);
#[repr(C)]
pub struct SPSTATEHANDLE__(i32);
#[repr(transparent)]
pub struct SPSTREAMFORMAT(pub i32);
pub const SPSF_Default: SPSTREAMFORMAT = SPSTREAMFORMAT(-1i32);
pub const SPSF_NoAssignedFormat: SPSTREAMFORMAT = SPSTREAMFORMAT(0i32);
pub const SPSF_Text: SPSTREAMFORMAT = SPSTREAMFORMAT(1i32);
pub const SPSF_NonStandardFormat: SPSTREAMFORMAT = SPSTREAMFORMAT(2i32);
pub const SPSF_ExtendedAudioFormat: SPSTREAMFORMAT = SPSTREAMFORMAT(3i32);
pub const SPSF_8kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(4i32);
pub const SPSF_8kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(5i32);
pub const SPSF_8kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(6i32);
pub const SPSF_8kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(7i32);
pub const SPSF_11kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(8i32);
pub const SPSF_11kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(9i32);
pub const SPSF_11kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(10i32);
pub const SPSF_11kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(11i32);
pub const SPSF_12kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(12i32);
pub const SPSF_12kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(13i32);
pub const SPSF_12kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(14i32);
pub const SPSF_12kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(15i32);
pub const SPSF_16kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(16i32);
pub const SPSF_16kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(17i32);
pub const SPSF_16kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(18i32);
pub const SPSF_16kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(19i32);
pub const SPSF_22kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(20i32);
pub const SPSF_22kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(21i32);
pub const SPSF_22kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(22i32);
pub const SPSF_22kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(23i32);
pub const SPSF_24kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(24i32);
pub const SPSF_24kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(25i32);
pub const SPSF_24kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(26i32);
pub const SPSF_24kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(27i32);
pub const SPSF_32kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(28i32);
pub const SPSF_32kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(29i32);
pub const SPSF_32kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(30i32);
pub const SPSF_32kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(31i32);
pub const SPSF_44kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(32i32);
pub const SPSF_44kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(33i32);
pub const SPSF_44kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(34i32);
pub const SPSF_44kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(35i32);
pub const SPSF_48kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(36i32);
pub const SPSF_48kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(37i32);
pub const SPSF_48kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(38i32);
pub const SPSF_48kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(39i32);
pub const SPSF_TrueSpeech_8kHz1BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(40i32);
pub const SPSF_CCITT_ALaw_8kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(41i32);
pub const SPSF_CCITT_ALaw_8kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(42i32);
pub const SPSF_CCITT_ALaw_11kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(43i32);
pub const SPSF_CCITT_ALaw_11kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(44i32);
pub const SPSF_CCITT_ALaw_22kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(45i32);
pub const SPSF_CCITT_ALaw_22kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(46i32);
pub const SPSF_CCITT_ALaw_44kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(47i32);
pub const SPSF_CCITT_ALaw_44kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(48i32);
pub const SPSF_CCITT_uLaw_8kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(49i32);
pub const SPSF_CCITT_uLaw_8kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(50i32);
pub const SPSF_CCITT_uLaw_11kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(51i32);
pub const SPSF_CCITT_uLaw_11kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(52i32);
pub const SPSF_CCITT_uLaw_22kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(53i32);
pub const SPSF_CCITT_uLaw_22kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(54i32);
pub const SPSF_CCITT_uLaw_44kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(55i32);
pub const SPSF_CCITT_uLaw_44kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(56i32);
pub const SPSF_ADPCM_8kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(57i32);
pub const SPSF_ADPCM_8kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(58i32);
pub const SPSF_ADPCM_11kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(59i32);
pub const SPSF_ADPCM_11kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(60i32);
pub const SPSF_ADPCM_22kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(61i32);
pub const SPSF_ADPCM_22kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(62i32);
pub const SPSF_ADPCM_44kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(63i32);
pub const SPSF_ADPCM_44kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(64i32);
pub const SPSF_GSM610_8kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(65i32);
pub const SPSF_GSM610_11kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(66i32);
pub const SPSF_GSM610_22kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(67i32);
pub const SPSF_GSM610_44kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(68i32);
pub const SPSF_NUM_FORMATS: SPSTREAMFORMAT = SPSTREAMFORMAT(69i32);
#[repr(C)]
pub struct SPTEXTSELECTIONINFO(i32);
#[repr(transparent)]
pub struct SPVACTIONS(pub i32);
pub const SPVA_Speak: SPVACTIONS = SPVACTIONS(0i32);
pub const SPVA_Silence: SPVACTIONS = SPVACTIONS(1i32);
pub const SPVA_Pronounce: SPVACTIONS = SPVACTIONS(2i32);
pub const SPVA_Bookmark: SPVACTIONS = SPVACTIONS(3i32);
pub const SPVA_SpellOut: SPVACTIONS = SPVACTIONS(4i32);
pub const SPVA_Section: SPVACTIONS = SPVACTIONS(5i32);
pub const SPVA_ParseUnknownTag: SPVACTIONS = SPVACTIONS(6i32);
#[repr(transparent)]
pub struct SPVALUETYPE(pub i32);
pub const SPDF_PROPERTY: SPVALUETYPE = SPVALUETYPE(1i32);
pub const SPDF_REPLACEMENT: SPVALUETYPE = SPVALUETYPE(2i32);
pub const SPDF_RULE: SPVALUETYPE = SPVALUETYPE(4i32);
pub const SPDF_DISPLAYTEXT: SPVALUETYPE = SPVALUETYPE(8i32);
pub const SPDF_LEXICALFORM: SPVALUETYPE = SPVALUETYPE(16i32);
pub const SPDF_PRONUNCIATION: SPVALUETYPE = SPVALUETYPE(32i32);
pub const SPDF_AUDIO: SPVALUETYPE = SPVALUETYPE(64i32);
pub const SPDF_ALTERNATES: SPVALUETYPE = SPVALUETYPE(128i32);
pub const SPDF_ALL: SPVALUETYPE = SPVALUETYPE(255i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SPVCONTEXT(i32);
#[repr(transparent)]
pub struct SPVFEATURE(pub i32);
pub const SPVFEATURE_STRESSED: SPVFEATURE = SPVFEATURE(1i32);
pub const SPVFEATURE_EMPHASIS: SPVFEATURE = SPVFEATURE(2i32);
#[repr(transparent)]
pub struct SPVISEMES(pub i32);
pub const SP_VISEME_0: SPVISEMES = SPVISEMES(0i32);
pub const SP_VISEME_1: SPVISEMES = SPVISEMES(1i32);
pub const SP_VISEME_2: SPVISEMES = SPVISEMES(2i32);
pub const SP_VISEME_3: SPVISEMES = SPVISEMES(3i32);
pub const SP_VISEME_4: SPVISEMES = SPVISEMES(4i32);
pub const SP_VISEME_5: SPVISEMES = SPVISEMES(5i32);
pub const SP_VISEME_6: SPVISEMES = SPVISEMES(6i32);
pub const SP_VISEME_7: SPVISEMES = SPVISEMES(7i32);
pub const SP_VISEME_8: SPVISEMES = SPVISEMES(8i32);
pub const SP_VISEME_9: SPVISEMES = SPVISEMES(9i32);
pub const SP_VISEME_10: SPVISEMES = SPVISEMES(10i32);
pub const SP_VISEME_11: SPVISEMES = SPVISEMES(11i32);
pub const SP_VISEME_12: SPVISEMES = SPVISEMES(12i32);
pub const SP_VISEME_13: SPVISEMES = SPVISEMES(13i32);
pub const SP_VISEME_14: SPVISEMES = SPVISEMES(14i32);
pub const SP_VISEME_15: SPVISEMES = SPVISEMES(15i32);
pub const SP_VISEME_16: SPVISEMES = SPVISEMES(16i32);
pub const SP_VISEME_17: SPVISEMES = SPVISEMES(17i32);
pub const SP_VISEME_18: SPVISEMES = SPVISEMES(18i32);
pub const SP_VISEME_19: SPVISEMES = SPVISEMES(19i32);
pub const SP_VISEME_20: SPVISEMES = SPVISEMES(20i32);
pub const SP_VISEME_21: SPVISEMES = SPVISEMES(21i32);
#[repr(transparent)]
pub struct SPVLIMITS(pub i32);
pub const SPMIN_VOLUME: SPVLIMITS = SPVLIMITS(0i32);
pub const SPMAX_VOLUME: SPVLIMITS = SPVLIMITS(100i32);
pub const SPMIN_RATE: SPVLIMITS = SPVLIMITS(-10i32);
pub const SPMAX_RATE: SPVLIMITS = SPVLIMITS(10i32);
#[repr(C)]
pub struct SPVOICESTATUS(i32);
#[repr(C)]
pub struct SPVPITCH(i32);
#[repr(transparent)]
pub struct SPVPRIORITY(pub i32);
pub const SPVPRI_NORMAL: SPVPRIORITY = SPVPRIORITY(0i32);
pub const SPVPRI_ALERT: SPVPRIORITY = SPVPRIORITY(1i32);
pub const SPVPRI_OVER: SPVPRIORITY = SPVPRIORITY(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SPVSTATE(i32);
#[repr(transparent)]
pub struct SPWAVEFORMATTYPE(pub i32);
pub const SPWF_INPUT: SPWAVEFORMATTYPE = SPWAVEFORMATTYPE(0i32);
pub const SPWF_SRENGINE: SPWAVEFORMATTYPE = SPWAVEFORMATTYPE(1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SPWORD(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SPWORDLIST(i32);
#[repr(transparent)]
pub struct SPWORDPRONOUNCEABLE(pub i32);
pub const SPWP_UNKNOWN_WORD_UNPRONOUNCEABLE: SPWORDPRONOUNCEABLE = SPWORDPRONOUNCEABLE(0i32);
pub const SPWP_UNKNOWN_WORD_PRONOUNCEABLE: SPWORDPRONOUNCEABLE = SPWORDPRONOUNCEABLE(1i32);
pub const SPWP_KNOWN_WORD_PRONOUNCEABLE: SPWORDPRONOUNCEABLE = SPWORDPRONOUNCEABLE(2i32);
#[repr(C)]
pub struct SPWORDPRONUNCIATION(i32);
#[repr(C)]
pub struct SPWORDPRONUNCIATIONLIST(i32);
#[repr(transparent)]
pub struct SPWORDTYPE(pub i32);
pub const eWORDTYPE_ADDED: SPWORDTYPE = SPWORDTYPE(1i32);
pub const eWORDTYPE_DELETED: SPWORDTYPE = SPWORDTYPE(2i32);
#[repr(transparent)]
pub struct SPXMLRESULTOPTIONS(pub i32);
pub const SPXRO_SML: SPXMLRESULTOPTIONS = SPXMLRESULTOPTIONS(0i32);
pub const SPXRO_Alternates_SML: SPXMLRESULTOPTIONS = SPXMLRESULTOPTIONS(1i32);
pub const SP_EMULATE_RESULT: u32 = 1073741824u32;
pub const SP_LOW_CONFIDENCE: i32 = -1i32;
pub const SP_MAX_LANGIDS: u32 = 20u32;
pub const SP_MAX_PRON_LENGTH: u32 = 384u32;
pub const SP_MAX_WORD_LENGTH: u32 = 128u32;
pub const SP_NORMAL_CONFIDENCE: u32 = 0u32;
pub const SP_STREAMPOS_ASAP: u32 = 0u32;
pub const SP_STREAMPOS_REALTIME: i32 = -1i32;
#[repr(C)]
pub struct SpAudioFormat(i32);
#[repr(C)]
pub struct SpCompressedLexicon(i32);
#[repr(C)]
pub struct SpCustomStream(i32);
#[repr(C)]
pub struct SpFileStream(i32);
#[repr(C)]
pub struct SpInProcRecoContext(i32);
#[repr(C)]
pub struct SpInprocRecognizer(i32);
#[repr(C)]
pub struct SpLexicon(i32);
#[repr(C)]
pub struct SpMMAudioEnum(i32);
#[repr(C)]
pub struct SpMMAudioIn(i32);
#[repr(C)]
pub struct SpMMAudioOut(i32);
#[repr(C)]
pub struct SpMemoryStream(i32);
#[repr(C)]
pub struct SpNotifyTranslator(i32);
#[repr(C)]
pub struct SpNullPhoneConverter(i32);
#[repr(C)]
pub struct SpObjectToken(i32);
#[repr(C)]
pub struct SpObjectTokenCategory(i32);
#[repr(C)]
pub struct SpPhoneConverter(i32);
#[repr(C)]
pub struct SpPhoneticAlphabetConverter(i32);
#[repr(C)]
pub struct SpPhraseInfoBuilder(i32);
#[repr(C)]
pub struct SpResourceManager(i32);
#[repr(C)]
pub struct SpSharedRecoContext(i32);
#[repr(C)]
pub struct SpSharedRecognizer(i32);
#[repr(C)]
pub struct SpShortcut(i32);
#[repr(C)]
pub struct SpStream(i32);
#[repr(C)]
pub struct SpStreamFormatConverter(i32);
#[repr(C)]
pub struct SpTextSelectionInformation(i32);
#[repr(C)]
pub struct SpUnCompressedLexicon(i32);
#[repr(C)]
pub struct SpVoice(i32);
#[repr(C)]
pub struct SpWaveFormatEx(i32);
pub const SpeechAllElements: i32 = -1i32;
#[repr(transparent)]
pub struct SpeechAudioFormatType(pub i32);
pub const SAFTDefault: SpeechAudioFormatType = SpeechAudioFormatType(-1i32);
pub const SAFTNoAssignedFormat: SpeechAudioFormatType = SpeechAudioFormatType(0i32);
pub const SAFTText: SpeechAudioFormatType = SpeechAudioFormatType(1i32);
pub const SAFTNonStandardFormat: SpeechAudioFormatType = SpeechAudioFormatType(2i32);
pub const SAFTExtendedAudioFormat: SpeechAudioFormatType = SpeechAudioFormatType(3i32);
pub const SAFT8kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(4i32);
pub const SAFT8kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(5i32);
pub const SAFT8kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(6i32);
pub const SAFT8kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(7i32);
pub const SAFT11kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(8i32);
pub const SAFT11kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(9i32);
pub const SAFT11kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(10i32);
pub const SAFT11kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(11i32);
pub const SAFT12kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(12i32);
pub const SAFT12kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(13i32);
pub const SAFT12kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(14i32);
pub const SAFT12kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(15i32);
pub const SAFT16kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(16i32);
pub const SAFT16kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(17i32);
pub const SAFT16kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(18i32);
pub const SAFT16kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(19i32);
pub const SAFT22kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(20i32);
pub const SAFT22kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(21i32);
pub const SAFT22kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(22i32);
pub const SAFT22kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(23i32);
pub const SAFT24kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(24i32);
pub const SAFT24kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(25i32);
pub const SAFT24kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(26i32);
pub const SAFT24kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(27i32);
pub const SAFT32kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(28i32);
pub const SAFT32kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(29i32);
pub const SAFT32kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(30i32);
pub const SAFT32kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(31i32);
pub const SAFT44kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(32i32);
pub const SAFT44kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(33i32);
pub const SAFT44kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(34i32);
pub const SAFT44kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(35i32);
pub const SAFT48kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(36i32);
pub const SAFT48kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(37i32);
pub const SAFT48kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(38i32);
pub const SAFT48kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(39i32);
pub const SAFTTrueSpeech_8kHz1BitMono: SpeechAudioFormatType = SpeechAudioFormatType(40i32);
pub const SAFTCCITT_ALaw_8kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(41i32);
pub const SAFTCCITT_ALaw_8kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(42i32);
pub const SAFTCCITT_ALaw_11kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(43i32);
pub const SAFTCCITT_ALaw_11kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(44i32);
pub const SAFTCCITT_ALaw_22kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(45i32);
pub const SAFTCCITT_ALaw_22kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(46i32);
pub const SAFTCCITT_ALaw_44kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(47i32);
pub const SAFTCCITT_ALaw_44kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(48i32);
pub const SAFTCCITT_uLaw_8kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(49i32);
pub const SAFTCCITT_uLaw_8kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(50i32);
pub const SAFTCCITT_uLaw_11kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(51i32);
pub const SAFTCCITT_uLaw_11kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(52i32);
pub const SAFTCCITT_uLaw_22kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(53i32);
pub const SAFTCCITT_uLaw_22kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(54i32);
pub const SAFTCCITT_uLaw_44kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(55i32);
pub const SAFTCCITT_uLaw_44kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(56i32);
pub const SAFTADPCM_8kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(57i32);
pub const SAFTADPCM_8kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(58i32);
pub const SAFTADPCM_11kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(59i32);
pub const SAFTADPCM_11kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(60i32);
pub const SAFTADPCM_22kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(61i32);
pub const SAFTADPCM_22kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(62i32);
pub const SAFTADPCM_44kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(63i32);
pub const SAFTADPCM_44kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(64i32);
pub const SAFTGSM610_8kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(65i32);
pub const SAFTGSM610_11kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(66i32);
pub const SAFTGSM610_22kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(67i32);
pub const SAFTGSM610_44kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(68i32);
#[repr(transparent)]
pub struct SpeechAudioState(pub i32);
pub const SASClosed: SpeechAudioState = SpeechAudioState(0i32);
pub const SASStop: SpeechAudioState = SpeechAudioState(1i32);
pub const SASPause: SpeechAudioState = SpeechAudioState(2i32);
pub const SASRun: SpeechAudioState = SpeechAudioState(3i32);
#[repr(transparent)]
pub struct SpeechBookmarkOptions(pub i32);
pub const SBONone: SpeechBookmarkOptions = SpeechBookmarkOptions(0i32);
pub const SBOPause: SpeechBookmarkOptions = SpeechBookmarkOptions(1i32);
#[repr(transparent)]
pub struct SpeechDataKeyLocation(pub i32);
pub const SDKLDefaultLocation: SpeechDataKeyLocation = SpeechDataKeyLocation(0i32);
pub const SDKLCurrentUser: SpeechDataKeyLocation = SpeechDataKeyLocation(1i32);
pub const SDKLLocalMachine: SpeechDataKeyLocation = SpeechDataKeyLocation(2i32);
pub const SDKLCurrentConfig: SpeechDataKeyLocation = SpeechDataKeyLocation(5i32);
#[repr(transparent)]
pub struct SpeechDiscardType(pub i32);
pub const SDTProperty: SpeechDiscardType = SpeechDiscardType(1i32);
pub const SDTReplacement: SpeechDiscardType = SpeechDiscardType(2i32);
pub const SDTRule: SpeechDiscardType = SpeechDiscardType(4i32);
pub const SDTDisplayText: SpeechDiscardType = SpeechDiscardType(8i32);
pub const SDTLexicalForm: SpeechDiscardType = SpeechDiscardType(16i32);
pub const SDTPronunciation: SpeechDiscardType = SpeechDiscardType(32i32);
pub const SDTAudio: SpeechDiscardType = SpeechDiscardType(64i32);
pub const SDTAlternates: SpeechDiscardType = SpeechDiscardType(128i32);
pub const SDTAll: SpeechDiscardType = SpeechDiscardType(255i32);
#[repr(transparent)]
pub struct SpeechDisplayAttributes(pub i32);
pub const SDA_No_Trailing_Space: SpeechDisplayAttributes = SpeechDisplayAttributes(0i32);
pub const SDA_One_Trailing_Space: SpeechDisplayAttributes = SpeechDisplayAttributes(2i32);
pub const SDA_Two_Trailing_Spaces: SpeechDisplayAttributes = SpeechDisplayAttributes(4i32);
pub const SDA_Consume_Leading_Spaces: SpeechDisplayAttributes = SpeechDisplayAttributes(8i32);
#[repr(transparent)]
pub struct SpeechEmulationCompareFlags(pub i32);
pub const SECFIgnoreCase: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(1i32);
pub const SECFIgnoreKanaType: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(65536i32);
pub const SECFIgnoreWidth: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(131072i32);
pub const SECFNoSpecialChars: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(536870912i32);
pub const SECFEmulateResult: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(1073741824i32);
pub const SECFDefault: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(196609i32);
#[repr(transparent)]
pub struct SpeechEngineConfidence(pub i32);
pub const SECLowConfidence: SpeechEngineConfidence = SpeechEngineConfidence(-1i32);
pub const SECNormalConfidence: SpeechEngineConfidence = SpeechEngineConfidence(0i32);
pub const SECHighConfidence: SpeechEngineConfidence = SpeechEngineConfidence(1i32);
#[repr(transparent)]
pub struct SpeechFormatType(pub i32);
pub const SFTInput: SpeechFormatType = SpeechFormatType(0i32);
pub const SFTSREngine: SpeechFormatType = SpeechFormatType(1i32);
#[repr(transparent)]
pub struct SpeechGrammarRuleStateTransitionType(pub i32);
pub const SGRSTTEpsilon: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(0i32);
pub const SGRSTTWord: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(1i32);
pub const SGRSTTRule: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(2i32);
pub const SGRSTTDictation: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(3i32);
pub const SGRSTTWildcard: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(4i32);
pub const SGRSTTTextBuffer: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(5i32);
#[repr(transparent)]
pub struct SpeechGrammarState(pub i32);
pub const SGSEnabled: SpeechGrammarState = SpeechGrammarState(1i32);
pub const SGSDisabled: SpeechGrammarState = SpeechGrammarState(0i32);
pub const SGSExclusive: SpeechGrammarState = SpeechGrammarState(3i32);
#[repr(transparent)]
pub struct SpeechGrammarWordType(pub i32);
pub const SGDisplay: SpeechGrammarWordType = SpeechGrammarWordType(0i32);
pub const SGLexical: SpeechGrammarWordType = SpeechGrammarWordType(1i32);
pub const SGPronounciation: SpeechGrammarWordType = SpeechGrammarWordType(2i32);
pub const SGLexicalNoSpecialChars: SpeechGrammarWordType = SpeechGrammarWordType(3i32);
#[repr(transparent)]
pub struct SpeechInterference(pub i32);
pub const SINone: SpeechInterference = SpeechInterference(0i32);
pub const SINoise: SpeechInterference = SpeechInterference(1i32);
pub const SINoSignal: SpeechInterference = SpeechInterference(2i32);
pub const SITooLoud: SpeechInterference = SpeechInterference(3i32);
pub const SITooQuiet: SpeechInterference = SpeechInterference(4i32);
pub const SITooFast: SpeechInterference = SpeechInterference(5i32);
pub const SITooSlow: SpeechInterference = SpeechInterference(6i32);
#[repr(transparent)]
pub struct SpeechLexiconType(pub i32);
pub const SLTUser: SpeechLexiconType = SpeechLexiconType(1i32);
pub const SLTApp: SpeechLexiconType = SpeechLexiconType(2i32);
#[repr(transparent)]
pub struct SpeechLoadOption(pub i32);
pub const SLOStatic: SpeechLoadOption = SpeechLoadOption(0i32);
pub const SLODynamic: SpeechLoadOption = SpeechLoadOption(1i32);
#[repr(transparent)]
pub struct SpeechPartOfSpeech(pub i32);
pub const SPSNotOverriden: SpeechPartOfSpeech = SpeechPartOfSpeech(-1i32);
pub const SPSUnknown: SpeechPartOfSpeech = SpeechPartOfSpeech(0i32);
pub const SPSNoun: SpeechPartOfSpeech = SpeechPartOfSpeech(4096i32);
pub const SPSVerb: SpeechPartOfSpeech = SpeechPartOfSpeech(8192i32);
pub const SPSModifier: SpeechPartOfSpeech = SpeechPartOfSpeech(12288i32);
pub const SPSFunction: SpeechPartOfSpeech = SpeechPartOfSpeech(16384i32);
pub const SPSInterjection: SpeechPartOfSpeech = SpeechPartOfSpeech(20480i32);
pub const SPSLMA: SpeechPartOfSpeech = SpeechPartOfSpeech(28672i32);
pub const SPSSuppressWord: SpeechPartOfSpeech = SpeechPartOfSpeech(61440i32);
#[repr(transparent)]
pub struct SpeechRecoContextState(pub i32);
pub const SRCS_Disabled: SpeechRecoContextState = SpeechRecoContextState(0i32);
pub const SRCS_Enabled: SpeechRecoContextState = SpeechRecoContextState(1i32);
#[repr(transparent)]
pub struct SpeechRecoEvents(pub i32);
pub const SREStreamEnd: SpeechRecoEvents = SpeechRecoEvents(1i32);
pub const SRESoundStart: SpeechRecoEvents = SpeechRecoEvents(2i32);
pub const SRESoundEnd: SpeechRecoEvents = SpeechRecoEvents(4i32);
pub const SREPhraseStart: SpeechRecoEvents = SpeechRecoEvents(8i32);
pub const SRERecognition: SpeechRecoEvents = SpeechRecoEvents(16i32);
pub const SREHypothesis: SpeechRecoEvents = SpeechRecoEvents(32i32);
pub const SREBookmark: SpeechRecoEvents = SpeechRecoEvents(64i32);
pub const SREPropertyNumChange: SpeechRecoEvents = SpeechRecoEvents(128i32);
pub const SREPropertyStringChange: SpeechRecoEvents = SpeechRecoEvents(256i32);
pub const SREFalseRecognition: SpeechRecoEvents = SpeechRecoEvents(512i32);
pub const SREInterference: SpeechRecoEvents = SpeechRecoEvents(1024i32);
pub const SRERequestUI: SpeechRecoEvents = SpeechRecoEvents(2048i32);
pub const SREStateChange: SpeechRecoEvents = SpeechRecoEvents(4096i32);
pub const SREAdaptation: SpeechRecoEvents = SpeechRecoEvents(8192i32);
pub const SREStreamStart: SpeechRecoEvents = SpeechRecoEvents(16384i32);
pub const SRERecoOtherContext: SpeechRecoEvents = SpeechRecoEvents(32768i32);
pub const SREAudioLevel: SpeechRecoEvents = SpeechRecoEvents(65536i32);
pub const SREPrivate: SpeechRecoEvents = SpeechRecoEvents(262144i32);
pub const SREAllEvents: SpeechRecoEvents = SpeechRecoEvents(393215i32);
#[repr(transparent)]
pub struct SpeechRecognitionType(pub i32);
pub const SRTStandard: SpeechRecognitionType = SpeechRecognitionType(0i32);
pub const SRTAutopause: SpeechRecognitionType = SpeechRecognitionType(1i32);
pub const SRTEmulated: SpeechRecognitionType = SpeechRecognitionType(2i32);
pub const SRTSMLTimeout: SpeechRecognitionType = SpeechRecognitionType(4i32);
pub const SRTExtendableParse: SpeechRecognitionType = SpeechRecognitionType(8i32);
pub const SRTReSent: SpeechRecognitionType = SpeechRecognitionType(16i32);
#[repr(transparent)]
pub struct SpeechRecognizerState(pub i32);
pub const SRSInactive: SpeechRecognizerState = SpeechRecognizerState(0i32);
pub const SRSActive: SpeechRecognizerState = SpeechRecognizerState(1i32);
pub const SRSActiveAlways: SpeechRecognizerState = SpeechRecognizerState(2i32);
pub const SRSInactiveWithPurge: SpeechRecognizerState = SpeechRecognizerState(3i32);
#[repr(transparent)]
pub struct SpeechRetainedAudioOptions(pub i32);
pub const SRAONone: SpeechRetainedAudioOptions = SpeechRetainedAudioOptions(0i32);
pub const SRAORetainAudio: SpeechRetainedAudioOptions = SpeechRetainedAudioOptions(1i32);
#[repr(transparent)]
pub struct SpeechRuleAttributes(pub i32);
pub const SRATopLevel: SpeechRuleAttributes = SpeechRuleAttributes(1i32);
pub const SRADefaultToActive: SpeechRuleAttributes = SpeechRuleAttributes(2i32);
pub const SRAExport: SpeechRuleAttributes = SpeechRuleAttributes(4i32);
pub const SRAImport: SpeechRuleAttributes = SpeechRuleAttributes(8i32);
pub const SRAInterpreter: SpeechRuleAttributes = SpeechRuleAttributes(16i32);
pub const SRADynamic: SpeechRuleAttributes = SpeechRuleAttributes(32i32);
pub const SRARoot: SpeechRuleAttributes = SpeechRuleAttributes(64i32);
#[repr(transparent)]
pub struct SpeechRuleState(pub i32);
pub const SGDSInactive: SpeechRuleState = SpeechRuleState(0i32);
pub const SGDSActive: SpeechRuleState = SpeechRuleState(1i32);
pub const SGDSActiveWithAutoPause: SpeechRuleState = SpeechRuleState(3i32);
pub const SGDSActiveUserDelimited: SpeechRuleState = SpeechRuleState(4i32);
#[repr(transparent)]
pub struct SpeechRunState(pub i32);
pub const SRSEDone: SpeechRunState = SpeechRunState(1i32);
pub const SRSEIsSpeaking: SpeechRunState = SpeechRunState(2i32);
#[repr(transparent)]
pub struct SpeechSpecialTransitionType(pub i32);
pub const SSTTWildcard: SpeechSpecialTransitionType = SpeechSpecialTransitionType(1i32);
pub const SSTTDictation: SpeechSpecialTransitionType = SpeechSpecialTransitionType(2i32);
pub const SSTTTextBuffer: SpeechSpecialTransitionType = SpeechSpecialTransitionType(3i32);
#[repr(transparent)]
pub struct SpeechStreamFileMode(pub i32);
pub const SSFMOpenForRead: SpeechStreamFileMode = SpeechStreamFileMode(0i32);
pub const SSFMOpenReadWrite: SpeechStreamFileMode = SpeechStreamFileMode(1i32);
pub const SSFMCreate: SpeechStreamFileMode = SpeechStreamFileMode(2i32);
pub const SSFMCreateForWrite: SpeechStreamFileMode = SpeechStreamFileMode(3i32);
#[repr(transparent)]
pub struct SpeechStreamSeekPositionType(pub u32);
pub const SSSPTRelativeToStart: SpeechStreamSeekPositionType = SpeechStreamSeekPositionType(0u32);
pub const SSSPTRelativeToCurrentPosition: SpeechStreamSeekPositionType = SpeechStreamSeekPositionType(1u32);
pub const SSSPTRelativeToEnd: SpeechStreamSeekPositionType = SpeechStreamSeekPositionType(2u32);
#[repr(transparent)]
pub struct SpeechTokenContext(pub u32);
pub const STCInprocServer: SpeechTokenContext = SpeechTokenContext(1u32);
pub const STCInprocHandler: SpeechTokenContext = SpeechTokenContext(2u32);
pub const STCLocalServer: SpeechTokenContext = SpeechTokenContext(4u32);
pub const STCRemoteServer: SpeechTokenContext = SpeechTokenContext(16u32);
pub const STCAll: SpeechTokenContext = SpeechTokenContext(23u32);
#[repr(transparent)]
pub struct SpeechTokenShellFolder(pub i32);
pub const STSF_AppData: SpeechTokenShellFolder = SpeechTokenShellFolder(26i32);
pub const STSF_LocalAppData: SpeechTokenShellFolder = SpeechTokenShellFolder(28i32);
pub const STSF_CommonAppData: SpeechTokenShellFolder = SpeechTokenShellFolder(35i32);
pub const STSF_FlagCreate: SpeechTokenShellFolder = SpeechTokenShellFolder(32768i32);
#[repr(transparent)]
pub struct SpeechVisemeFeature(pub i32);
pub const SVF_None: SpeechVisemeFeature = SpeechVisemeFeature(0i32);
pub const SVF_Stressed: SpeechVisemeFeature = SpeechVisemeFeature(1i32);
pub const SVF_Emphasis: SpeechVisemeFeature = SpeechVisemeFeature(2i32);
#[repr(transparent)]
pub struct SpeechVisemeType(pub i32);
pub const SVP_0: SpeechVisemeType = SpeechVisemeType(0i32);
pub const SVP_1: SpeechVisemeType = SpeechVisemeType(1i32);
pub const SVP_2: SpeechVisemeType = SpeechVisemeType(2i32);
pub const SVP_3: SpeechVisemeType = SpeechVisemeType(3i32);
pub const SVP_4: SpeechVisemeType = SpeechVisemeType(4i32);
pub const SVP_5: SpeechVisemeType = SpeechVisemeType(5i32);
pub const SVP_6: SpeechVisemeType = SpeechVisemeType(6i32);
pub const SVP_7: SpeechVisemeType = SpeechVisemeType(7i32);
pub const SVP_8: SpeechVisemeType = SpeechVisemeType(8i32);
pub const SVP_9: SpeechVisemeType = SpeechVisemeType(9i32);
pub const SVP_10: SpeechVisemeType = SpeechVisemeType(10i32);
pub const SVP_11: SpeechVisemeType = SpeechVisemeType(11i32);
pub const SVP_12: SpeechVisemeType = SpeechVisemeType(12i32);
pub const SVP_13: SpeechVisemeType = SpeechVisemeType(13i32);
pub const SVP_14: SpeechVisemeType = SpeechVisemeType(14i32);
pub const SVP_15: SpeechVisemeType = SpeechVisemeType(15i32);
pub const SVP_16: SpeechVisemeType = SpeechVisemeType(16i32);
pub const SVP_17: SpeechVisemeType = SpeechVisemeType(17i32);
pub const SVP_18: SpeechVisemeType = SpeechVisemeType(18i32);
pub const SVP_19: SpeechVisemeType = SpeechVisemeType(19i32);
pub const SVP_20: SpeechVisemeType = SpeechVisemeType(20i32);
pub const SVP_21: SpeechVisemeType = SpeechVisemeType(21i32);
#[repr(transparent)]
pub struct SpeechVoiceEvents(pub i32);
pub const SVEStartInputStream: SpeechVoiceEvents = SpeechVoiceEvents(2i32);
pub const SVEEndInputStream: SpeechVoiceEvents = SpeechVoiceEvents(4i32);
pub const SVEVoiceChange: SpeechVoiceEvents = SpeechVoiceEvents(8i32);
pub const SVEBookmark: SpeechVoiceEvents = SpeechVoiceEvents(16i32);
pub const SVEWordBoundary: SpeechVoiceEvents = SpeechVoiceEvents(32i32);
pub const SVEPhoneme: SpeechVoiceEvents = SpeechVoiceEvents(64i32);
pub const SVESentenceBoundary: SpeechVoiceEvents = SpeechVoiceEvents(128i32);
pub const SVEViseme: SpeechVoiceEvents = SpeechVoiceEvents(256i32);
pub const SVEAudioLevel: SpeechVoiceEvents = SpeechVoiceEvents(512i32);
pub const SVEPrivate: SpeechVoiceEvents = SpeechVoiceEvents(32768i32);
pub const SVEAllEvents: SpeechVoiceEvents = SpeechVoiceEvents(33790i32);
#[repr(transparent)]
pub struct SpeechVoicePriority(pub i32);
pub const SVPNormal: SpeechVoicePriority = SpeechVoicePriority(0i32);
pub const SVPAlert: SpeechVoicePriority = SpeechVoicePriority(1i32);
pub const SVPOver: SpeechVoicePriority = SpeechVoicePriority(2i32);
#[repr(transparent)]
pub struct SpeechVoiceSpeakFlags(pub i32);
pub const SVSFDefault: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(0i32);
pub const SVSFlagsAsync: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(1i32);
pub const SVSFPurgeBeforeSpeak: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(2i32);
pub const SVSFIsFilename: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(4i32);
pub const SVSFIsXML: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(8i32);
pub const SVSFIsNotXML: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(16i32);
pub const SVSFPersistXML: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(32i32);
pub const SVSFNLPSpeakPunc: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(64i32);
pub const SVSFParseSapi: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(128i32);
pub const SVSFParseSsml: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(256i32);
pub const SVSFParseAutodetect: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(0i32);
pub const SVSFNLPMask: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(64i32);
pub const SVSFParseMask: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(384i32);
pub const SVSFVoiceMask: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(511i32);
pub const SVSFUnusedFlags: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(-512i32);
#[repr(transparent)]
pub struct SpeechWordPronounceable(pub i32);
pub const SWPUnknownWordUnpronounceable: SpeechWordPronounceable = SpeechWordPronounceable(0i32);
pub const SWPUnknownWordPronounceable: SpeechWordPronounceable = SpeechWordPronounceable(1i32);
pub const SWPKnownWordPronounceable: SpeechWordPronounceable = SpeechWordPronounceable(2i32);
#[repr(transparent)]
pub struct SpeechWordType(pub i32);
pub const SWTAdded: SpeechWordType = SpeechWordType(1i32);
pub const SWTDeleted: SpeechWordType = SpeechWordType(2i32);
pub const Speech_Default_Weight: f32 = 1f32;
pub const Speech_Max_Pron_Length: i32 = 384i32;
pub const Speech_Max_Word_Length: i32 = 128i32;
pub const Speech_StreamPos_Asap: i32 = 0i32;
pub const Speech_StreamPos_RealTime: i32 = -1i32;
#[repr(transparent)]
pub struct _ISpeechRecoContextEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct _ISpeechVoiceEvents(pub *mut ::core::ffi::c_void);
