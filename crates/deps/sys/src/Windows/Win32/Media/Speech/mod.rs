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
impl ::core::marker::Copy for DISPIDSPRG {}
impl ::core::clone::Clone for DISPIDSPRG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPIDSPTSI(pub i32);
pub const DISPIDSPTSI_ActiveOffset: DISPIDSPTSI = DISPIDSPTSI(1i32);
pub const DISPIDSPTSI_ActiveLength: DISPIDSPTSI = DISPIDSPTSI(2i32);
pub const DISPIDSPTSI_SelectionOffset: DISPIDSPTSI = DISPIDSPTSI(3i32);
pub const DISPIDSPTSI_SelectionLength: DISPIDSPTSI = DISPIDSPTSI(4i32);
impl ::core::marker::Copy for DISPIDSPTSI {}
impl ::core::clone::Clone for DISPIDSPTSI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechAudio(pub i32);
pub const DISPID_SAStatus: DISPID_SpeechAudio = DISPID_SpeechAudio(200i32);
pub const DISPID_SABufferInfo: DISPID_SpeechAudio = DISPID_SpeechAudio(201i32);
pub const DISPID_SADefaultFormat: DISPID_SpeechAudio = DISPID_SpeechAudio(202i32);
pub const DISPID_SAVolume: DISPID_SpeechAudio = DISPID_SpeechAudio(203i32);
pub const DISPID_SABufferNotifySize: DISPID_SpeechAudio = DISPID_SpeechAudio(204i32);
pub const DISPID_SAEventHandle: DISPID_SpeechAudio = DISPID_SpeechAudio(205i32);
pub const DISPID_SASetState: DISPID_SpeechAudio = DISPID_SpeechAudio(206i32);
impl ::core::marker::Copy for DISPID_SpeechAudio {}
impl ::core::clone::Clone for DISPID_SpeechAudio {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechAudioBufferInfo(pub i32);
pub const DISPID_SABIMinNotification: DISPID_SpeechAudioBufferInfo = DISPID_SpeechAudioBufferInfo(1i32);
pub const DISPID_SABIBufferSize: DISPID_SpeechAudioBufferInfo = DISPID_SpeechAudioBufferInfo(2i32);
pub const DISPID_SABIEventBias: DISPID_SpeechAudioBufferInfo = DISPID_SpeechAudioBufferInfo(3i32);
impl ::core::marker::Copy for DISPID_SpeechAudioBufferInfo {}
impl ::core::clone::Clone for DISPID_SpeechAudioBufferInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechAudioFormat(pub i32);
pub const DISPID_SAFType: DISPID_SpeechAudioFormat = DISPID_SpeechAudioFormat(1i32);
pub const DISPID_SAFGuid: DISPID_SpeechAudioFormat = DISPID_SpeechAudioFormat(2i32);
pub const DISPID_SAFGetWaveFormatEx: DISPID_SpeechAudioFormat = DISPID_SpeechAudioFormat(3i32);
pub const DISPID_SAFSetWaveFormatEx: DISPID_SpeechAudioFormat = DISPID_SpeechAudioFormat(4i32);
impl ::core::marker::Copy for DISPID_SpeechAudioFormat {}
impl ::core::clone::Clone for DISPID_SpeechAudioFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechAudioStatus(pub i32);
pub const DISPID_SASFreeBufferSpace: DISPID_SpeechAudioStatus = DISPID_SpeechAudioStatus(1i32);
pub const DISPID_SASNonBlockingIO: DISPID_SpeechAudioStatus = DISPID_SpeechAudioStatus(2i32);
pub const DISPID_SASState: DISPID_SpeechAudioStatus = DISPID_SpeechAudioStatus(3i32);
pub const DISPID_SASCurrentSeekPosition: DISPID_SpeechAudioStatus = DISPID_SpeechAudioStatus(4i32);
pub const DISPID_SASCurrentDevicePosition: DISPID_SpeechAudioStatus = DISPID_SpeechAudioStatus(5i32);
impl ::core::marker::Copy for DISPID_SpeechAudioStatus {}
impl ::core::clone::Clone for DISPID_SpeechAudioStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechBaseStream(pub i32);
pub const DISPID_SBSFormat: DISPID_SpeechBaseStream = DISPID_SpeechBaseStream(1i32);
pub const DISPID_SBSRead: DISPID_SpeechBaseStream = DISPID_SpeechBaseStream(2i32);
pub const DISPID_SBSWrite: DISPID_SpeechBaseStream = DISPID_SpeechBaseStream(3i32);
pub const DISPID_SBSSeek: DISPID_SpeechBaseStream = DISPID_SpeechBaseStream(4i32);
impl ::core::marker::Copy for DISPID_SpeechBaseStream {}
impl ::core::clone::Clone for DISPID_SpeechBaseStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechCustomStream(pub i32);
pub const DISPID_SCSBaseStream: DISPID_SpeechCustomStream = DISPID_SpeechCustomStream(100i32);
impl ::core::marker::Copy for DISPID_SpeechCustomStream {}
impl ::core::clone::Clone for DISPID_SpeechCustomStream {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for DISPID_SpeechDataKey {}
impl ::core::clone::Clone for DISPID_SpeechDataKey {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechFileStream(pub i32);
pub const DISPID_SFSOpen: DISPID_SpeechFileStream = DISPID_SpeechFileStream(100i32);
pub const DISPID_SFSClose: DISPID_SpeechFileStream = DISPID_SpeechFileStream(101i32);
impl ::core::marker::Copy for DISPID_SpeechFileStream {}
impl ::core::clone::Clone for DISPID_SpeechFileStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechGrammarRule(pub i32);
pub const DISPID_SGRAttributes: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(1i32);
pub const DISPID_SGRInitialState: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(2i32);
pub const DISPID_SGRName: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(3i32);
pub const DISPID_SGRId: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(4i32);
pub const DISPID_SGRClear: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(5i32);
pub const DISPID_SGRAddResource: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(6i32);
pub const DISPID_SGRAddState: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(7i32);
impl ::core::marker::Copy for DISPID_SpeechGrammarRule {}
impl ::core::clone::Clone for DISPID_SpeechGrammarRule {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechGrammarRuleState(pub i32);
pub const DISPID_SGRSRule: DISPID_SpeechGrammarRuleState = DISPID_SpeechGrammarRuleState(1i32);
pub const DISPID_SGRSTransitions: DISPID_SpeechGrammarRuleState = DISPID_SpeechGrammarRuleState(2i32);
pub const DISPID_SGRSAddWordTransition: DISPID_SpeechGrammarRuleState = DISPID_SpeechGrammarRuleState(3i32);
pub const DISPID_SGRSAddRuleTransition: DISPID_SpeechGrammarRuleState = DISPID_SpeechGrammarRuleState(4i32);
pub const DISPID_SGRSAddSpecialTransition: DISPID_SpeechGrammarRuleState = DISPID_SpeechGrammarRuleState(5i32);
impl ::core::marker::Copy for DISPID_SpeechGrammarRuleState {}
impl ::core::clone::Clone for DISPID_SpeechGrammarRuleState {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for DISPID_SpeechGrammarRuleStateTransition {}
impl ::core::clone::Clone for DISPID_SpeechGrammarRuleStateTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechGrammarRuleStateTransitions(pub i32);
pub const DISPID_SGRSTsCount: DISPID_SpeechGrammarRuleStateTransitions = DISPID_SpeechGrammarRuleStateTransitions(1i32);
pub const DISPID_SGRSTsItem: DISPID_SpeechGrammarRuleStateTransitions = DISPID_SpeechGrammarRuleStateTransitions(0i32);
pub const DISPID_SGRSTs_NewEnum: DISPID_SpeechGrammarRuleStateTransitions = DISPID_SpeechGrammarRuleStateTransitions(-4i32);
impl ::core::marker::Copy for DISPID_SpeechGrammarRuleStateTransitions {}
impl ::core::clone::Clone for DISPID_SpeechGrammarRuleStateTransitions {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for DISPID_SpeechGrammarRules {}
impl ::core::clone::Clone for DISPID_SpeechGrammarRules {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for DISPID_SpeechLexicon {}
impl ::core::clone::Clone for DISPID_SpeechLexicon {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechLexiconProns(pub i32);
pub const DISPID_SLPsCount: DISPID_SpeechLexiconProns = DISPID_SpeechLexiconProns(1i32);
pub const DISPID_SLPsItem: DISPID_SpeechLexiconProns = DISPID_SpeechLexiconProns(0i32);
pub const DISPID_SLPs_NewEnum: DISPID_SpeechLexiconProns = DISPID_SpeechLexiconProns(-4i32);
impl ::core::marker::Copy for DISPID_SpeechLexiconProns {}
impl ::core::clone::Clone for DISPID_SpeechLexiconProns {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechLexiconPronunciation(pub i32);
pub const DISPID_SLPType: DISPID_SpeechLexiconPronunciation = DISPID_SpeechLexiconPronunciation(1i32);
pub const DISPID_SLPLangId: DISPID_SpeechLexiconPronunciation = DISPID_SpeechLexiconPronunciation(2i32);
pub const DISPID_SLPPartOfSpeech: DISPID_SpeechLexiconPronunciation = DISPID_SpeechLexiconPronunciation(3i32);
pub const DISPID_SLPPhoneIds: DISPID_SpeechLexiconPronunciation = DISPID_SpeechLexiconPronunciation(4i32);
pub const DISPID_SLPSymbolic: DISPID_SpeechLexiconPronunciation = DISPID_SpeechLexiconPronunciation(5i32);
impl ::core::marker::Copy for DISPID_SpeechLexiconPronunciation {}
impl ::core::clone::Clone for DISPID_SpeechLexiconPronunciation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechLexiconWord(pub i32);
pub const DISPID_SLWLangId: DISPID_SpeechLexiconWord = DISPID_SpeechLexiconWord(1i32);
pub const DISPID_SLWType: DISPID_SpeechLexiconWord = DISPID_SpeechLexiconWord(2i32);
pub const DISPID_SLWWord: DISPID_SpeechLexiconWord = DISPID_SpeechLexiconWord(3i32);
pub const DISPID_SLWPronunciations: DISPID_SpeechLexiconWord = DISPID_SpeechLexiconWord(4i32);
impl ::core::marker::Copy for DISPID_SpeechLexiconWord {}
impl ::core::clone::Clone for DISPID_SpeechLexiconWord {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechLexiconWords(pub i32);
pub const DISPID_SLWsCount: DISPID_SpeechLexiconWords = DISPID_SpeechLexiconWords(1i32);
pub const DISPID_SLWsItem: DISPID_SpeechLexiconWords = DISPID_SpeechLexiconWords(0i32);
pub const DISPID_SLWs_NewEnum: DISPID_SpeechLexiconWords = DISPID_SpeechLexiconWords(-4i32);
impl ::core::marker::Copy for DISPID_SpeechLexiconWords {}
impl ::core::clone::Clone for DISPID_SpeechLexiconWords {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechMMSysAudio(pub i32);
pub const DISPID_SMSADeviceId: DISPID_SpeechMMSysAudio = DISPID_SpeechMMSysAudio(300i32);
pub const DISPID_SMSALineId: DISPID_SpeechMMSysAudio = DISPID_SpeechMMSysAudio(301i32);
pub const DISPID_SMSAMMHandle: DISPID_SpeechMMSysAudio = DISPID_SpeechMMSysAudio(302i32);
impl ::core::marker::Copy for DISPID_SpeechMMSysAudio {}
impl ::core::clone::Clone for DISPID_SpeechMMSysAudio {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechMemoryStream(pub i32);
pub const DISPID_SMSSetData: DISPID_SpeechMemoryStream = DISPID_SpeechMemoryStream(100i32);
pub const DISPID_SMSGetData: DISPID_SpeechMemoryStream = DISPID_SpeechMemoryStream(101i32);
impl ::core::marker::Copy for DISPID_SpeechMemoryStream {}
impl ::core::clone::Clone for DISPID_SpeechMemoryStream {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for DISPID_SpeechObjectToken {}
impl ::core::clone::Clone for DISPID_SpeechObjectToken {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechObjectTokenCategory(pub i32);
pub const DISPID_SOTCId: DISPID_SpeechObjectTokenCategory = DISPID_SpeechObjectTokenCategory(1i32);
pub const DISPID_SOTCDefault: DISPID_SpeechObjectTokenCategory = DISPID_SpeechObjectTokenCategory(2i32);
pub const DISPID_SOTCSetId: DISPID_SpeechObjectTokenCategory = DISPID_SpeechObjectTokenCategory(3i32);
pub const DISPID_SOTCGetDataKey: DISPID_SpeechObjectTokenCategory = DISPID_SpeechObjectTokenCategory(4i32);
pub const DISPID_SOTCEnumerateTokens: DISPID_SpeechObjectTokenCategory = DISPID_SpeechObjectTokenCategory(5i32);
impl ::core::marker::Copy for DISPID_SpeechObjectTokenCategory {}
impl ::core::clone::Clone for DISPID_SpeechObjectTokenCategory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechObjectTokens(pub i32);
pub const DISPID_SOTsCount: DISPID_SpeechObjectTokens = DISPID_SpeechObjectTokens(1i32);
pub const DISPID_SOTsItem: DISPID_SpeechObjectTokens = DISPID_SpeechObjectTokens(0i32);
pub const DISPID_SOTs_NewEnum: DISPID_SpeechObjectTokens = DISPID_SpeechObjectTokens(-4i32);
impl ::core::marker::Copy for DISPID_SpeechObjectTokens {}
impl ::core::clone::Clone for DISPID_SpeechObjectTokens {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechPhoneConverter(pub i32);
pub const DISPID_SPCLangId: DISPID_SpeechPhoneConverter = DISPID_SpeechPhoneConverter(1i32);
pub const DISPID_SPCPhoneToId: DISPID_SpeechPhoneConverter = DISPID_SpeechPhoneConverter(2i32);
pub const DISPID_SPCIdToPhone: DISPID_SpeechPhoneConverter = DISPID_SpeechPhoneConverter(3i32);
impl ::core::marker::Copy for DISPID_SpeechPhoneConverter {}
impl ::core::clone::Clone for DISPID_SpeechPhoneConverter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechPhraseAlternate(pub i32);
pub const DISPID_SPARecoResult: DISPID_SpeechPhraseAlternate = DISPID_SpeechPhraseAlternate(1i32);
pub const DISPID_SPAStartElementInResult: DISPID_SpeechPhraseAlternate = DISPID_SpeechPhraseAlternate(2i32);
pub const DISPID_SPANumberOfElementsInResult: DISPID_SpeechPhraseAlternate = DISPID_SpeechPhraseAlternate(3i32);
pub const DISPID_SPAPhraseInfo: DISPID_SpeechPhraseAlternate = DISPID_SpeechPhraseAlternate(4i32);
pub const DISPID_SPACommit: DISPID_SpeechPhraseAlternate = DISPID_SpeechPhraseAlternate(5i32);
impl ::core::marker::Copy for DISPID_SpeechPhraseAlternate {}
impl ::core::clone::Clone for DISPID_SpeechPhraseAlternate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechPhraseAlternates(pub i32);
pub const DISPID_SPAsCount: DISPID_SpeechPhraseAlternates = DISPID_SpeechPhraseAlternates(1i32);
pub const DISPID_SPAsItem: DISPID_SpeechPhraseAlternates = DISPID_SpeechPhraseAlternates(0i32);
pub const DISPID_SPAs_NewEnum: DISPID_SpeechPhraseAlternates = DISPID_SpeechPhraseAlternates(-4i32);
impl ::core::marker::Copy for DISPID_SpeechPhraseAlternates {}
impl ::core::clone::Clone for DISPID_SpeechPhraseAlternates {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechPhraseBuilder(pub i32);
pub const DISPID_SPPBRestorePhraseFromMemory: DISPID_SpeechPhraseBuilder = DISPID_SpeechPhraseBuilder(1i32);
impl ::core::marker::Copy for DISPID_SpeechPhraseBuilder {}
impl ::core::clone::Clone for DISPID_SpeechPhraseBuilder {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for DISPID_SpeechPhraseElement {}
impl ::core::clone::Clone for DISPID_SpeechPhraseElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechPhraseElements(pub i32);
pub const DISPID_SPEsCount: DISPID_SpeechPhraseElements = DISPID_SpeechPhraseElements(1i32);
pub const DISPID_SPEsItem: DISPID_SpeechPhraseElements = DISPID_SpeechPhraseElements(0i32);
pub const DISPID_SPEs_NewEnum: DISPID_SpeechPhraseElements = DISPID_SpeechPhraseElements(-4i32);
impl ::core::marker::Copy for DISPID_SpeechPhraseElements {}
impl ::core::clone::Clone for DISPID_SpeechPhraseElements {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for DISPID_SpeechPhraseInfo {}
impl ::core::clone::Clone for DISPID_SpeechPhraseInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechPhraseProperties(pub i32);
pub const DISPID_SPPsCount: DISPID_SpeechPhraseProperties = DISPID_SpeechPhraseProperties(1i32);
pub const DISPID_SPPsItem: DISPID_SpeechPhraseProperties = DISPID_SpeechPhraseProperties(0i32);
pub const DISPID_SPPs_NewEnum: DISPID_SpeechPhraseProperties = DISPID_SpeechPhraseProperties(-4i32);
impl ::core::marker::Copy for DISPID_SpeechPhraseProperties {}
impl ::core::clone::Clone for DISPID_SpeechPhraseProperties {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for DISPID_SpeechPhraseProperty {}
impl ::core::clone::Clone for DISPID_SpeechPhraseProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechPhraseReplacement(pub i32);
pub const DISPID_SPRDisplayAttributes: DISPID_SpeechPhraseReplacement = DISPID_SpeechPhraseReplacement(1i32);
pub const DISPID_SPRText: DISPID_SpeechPhraseReplacement = DISPID_SpeechPhraseReplacement(2i32);
pub const DISPID_SPRFirstElement: DISPID_SpeechPhraseReplacement = DISPID_SpeechPhraseReplacement(3i32);
pub const DISPID_SPRNumberOfElements: DISPID_SpeechPhraseReplacement = DISPID_SpeechPhraseReplacement(4i32);
impl ::core::marker::Copy for DISPID_SpeechPhraseReplacement {}
impl ::core::clone::Clone for DISPID_SpeechPhraseReplacement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechPhraseReplacements(pub i32);
pub const DISPID_SPRsCount: DISPID_SpeechPhraseReplacements = DISPID_SpeechPhraseReplacements(1i32);
pub const DISPID_SPRsItem: DISPID_SpeechPhraseReplacements = DISPID_SpeechPhraseReplacements(0i32);
pub const DISPID_SPRs_NewEnum: DISPID_SpeechPhraseReplacements = DISPID_SpeechPhraseReplacements(-4i32);
impl ::core::marker::Copy for DISPID_SpeechPhraseReplacements {}
impl ::core::clone::Clone for DISPID_SpeechPhraseReplacements {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for DISPID_SpeechPhraseRule {}
impl ::core::clone::Clone for DISPID_SpeechPhraseRule {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechPhraseRules(pub i32);
pub const DISPID_SPRulesCount: DISPID_SpeechPhraseRules = DISPID_SpeechPhraseRules(1i32);
pub const DISPID_SPRulesItem: DISPID_SpeechPhraseRules = DISPID_SpeechPhraseRules(0i32);
pub const DISPID_SPRules_NewEnum: DISPID_SpeechPhraseRules = DISPID_SpeechPhraseRules(-4i32);
impl ::core::marker::Copy for DISPID_SpeechPhraseRules {}
impl ::core::clone::Clone for DISPID_SpeechPhraseRules {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for DISPID_SpeechRecoContext {}
impl ::core::clone::Clone for DISPID_SpeechRecoContext {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for DISPID_SpeechRecoContextEvents {}
impl ::core::clone::Clone for DISPID_SpeechRecoContextEvents {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for DISPID_SpeechRecoResult {}
impl ::core::clone::Clone for DISPID_SpeechRecoResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechRecoResult2(pub i32);
pub const DISPID_SRRSetTextFeedback: DISPID_SpeechRecoResult2 = DISPID_SpeechRecoResult2(12i32);
impl ::core::marker::Copy for DISPID_SpeechRecoResult2 {}
impl ::core::clone::Clone for DISPID_SpeechRecoResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechRecoResultTimes(pub i32);
pub const DISPID_SRRTStreamTime: DISPID_SpeechRecoResultTimes = DISPID_SpeechRecoResultTimes(1i32);
pub const DISPID_SRRTLength: DISPID_SpeechRecoResultTimes = DISPID_SpeechRecoResultTimes(2i32);
pub const DISPID_SRRTTickCount: DISPID_SpeechRecoResultTimes = DISPID_SpeechRecoResultTimes(3i32);
pub const DISPID_SRRTOffsetFromStart: DISPID_SpeechRecoResultTimes = DISPID_SpeechRecoResultTimes(4i32);
impl ::core::marker::Copy for DISPID_SpeechRecoResultTimes {}
impl ::core::clone::Clone for DISPID_SpeechRecoResultTimes {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for DISPID_SpeechRecognizer {}
impl ::core::clone::Clone for DISPID_SpeechRecognizer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechRecognizerStatus(pub i32);
pub const DISPID_SRSAudioStatus: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(1i32);
pub const DISPID_SRSCurrentStreamPosition: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(2i32);
pub const DISPID_SRSCurrentStreamNumber: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(3i32);
pub const DISPID_SRSNumberOfActiveRules: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(4i32);
pub const DISPID_SRSClsidEngine: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(5i32);
pub const DISPID_SRSSupportedLanguages: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(6i32);
impl ::core::marker::Copy for DISPID_SpeechRecognizerStatus {}
impl ::core::clone::Clone for DISPID_SpeechRecognizerStatus {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for DISPID_SpeechVoice {}
impl ::core::clone::Clone for DISPID_SpeechVoice {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for DISPID_SpeechVoiceEvent {}
impl ::core::clone::Clone for DISPID_SpeechVoiceEvent {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for DISPID_SpeechVoiceStatus {}
impl ::core::clone::Clone for DISPID_SpeechVoiceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechWaveFormatEx(pub i32);
pub const DISPID_SWFEFormatTag: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(1i32);
pub const DISPID_SWFEChannels: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(2i32);
pub const DISPID_SWFESamplesPerSec: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(3i32);
pub const DISPID_SWFEAvgBytesPerSec: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(4i32);
pub const DISPID_SWFEBlockAlign: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(5i32);
pub const DISPID_SWFEBitsPerSample: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(6i32);
pub const DISPID_SWFEExtraData: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(7i32);
impl ::core::marker::Copy for DISPID_SpeechWaveFormatEx {}
impl ::core::clone::Clone for DISPID_SpeechWaveFormatEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_SpeechXMLRecoResult(pub i32);
pub const DISPID_SRRGetXMLResult: DISPID_SpeechXMLRecoResult = DISPID_SpeechXMLRecoResult(10i32);
pub const DISPID_SRRGetXMLErrorInfo: DISPID_SpeechXMLRecoResult = DISPID_SpeechXMLRecoResult(11i32);
impl ::core::marker::Copy for DISPID_SpeechXMLRecoResult {}
impl ::core::clone::Clone for DISPID_SpeechXMLRecoResult {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PHONETICALPHABET {}
impl ::core::clone::Clone for PHONETICALPHABET {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SAPI_ERROR_BASE: u32 = 20480u32;
#[repr(transparent)]
pub struct SPADAPTATIONRELEVANCE(pub i32);
pub const SPAR_Unknown: SPADAPTATIONRELEVANCE = SPADAPTATIONRELEVANCE(0i32);
pub const SPAR_Low: SPADAPTATIONRELEVANCE = SPADAPTATIONRELEVANCE(1i32);
pub const SPAR_Medium: SPADAPTATIONRELEVANCE = SPADAPTATIONRELEVANCE(2i32);
pub const SPAR_High: SPADAPTATIONRELEVANCE = SPADAPTATIONRELEVANCE(3i32);
impl ::core::marker::Copy for SPADAPTATIONRELEVANCE {}
impl ::core::clone::Clone for SPADAPTATIONRELEVANCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SPADAPTATIONSETTINGS(pub i32);
pub const SPADS_Default: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(0i32);
pub const SPADS_CurrentRecognizer: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(1i32);
pub const SPADS_RecoProfile: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(2i32);
pub const SPADS_Immediate: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(4i32);
pub const SPADS_Reset: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(8i32);
pub const SPADS_HighVolumeDataSource: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(16i32);
impl ::core::marker::Copy for SPADAPTATIONSETTINGS {}
impl ::core::clone::Clone for SPADAPTATIONSETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SPAUDIOOPTIONS(pub i32);
pub const SPAO_NONE: SPAUDIOOPTIONS = SPAUDIOOPTIONS(0i32);
pub const SPAO_RETAIN_AUDIO: SPAUDIOOPTIONS = SPAUDIOOPTIONS(1i32);
impl ::core::marker::Copy for SPAUDIOOPTIONS {}
impl ::core::clone::Clone for SPAUDIOOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SPAUDIOSTATE(pub i32);
pub const SPAS_CLOSED: SPAUDIOSTATE = SPAUDIOSTATE(0i32);
pub const SPAS_STOP: SPAUDIOSTATE = SPAUDIOSTATE(1i32);
pub const SPAS_PAUSE: SPAUDIOSTATE = SPAUDIOSTATE(2i32);
pub const SPAS_RUN: SPAUDIOSTATE = SPAUDIOSTATE(3i32);
impl ::core::marker::Copy for SPAUDIOSTATE {}
impl ::core::clone::Clone for SPAUDIOSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SPBOOKMARKOPTIONS(pub i32);
pub const SPBO_NONE: SPBOOKMARKOPTIONS = SPBOOKMARKOPTIONS(0i32);
pub const SPBO_PAUSE: SPBOOKMARKOPTIONS = SPBOOKMARKOPTIONS(1i32);
pub const SPBO_AHEAD: SPBOOKMARKOPTIONS = SPBOOKMARKOPTIONS(2i32);
pub const SPBO_TIME_UNITS: SPBOOKMARKOPTIONS = SPBOOKMARKOPTIONS(4i32);
impl ::core::marker::Copy for SPBOOKMARKOPTIONS {}
impl ::core::clone::Clone for SPBOOKMARKOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SPCFGRULEATTRIBUTES {}
impl ::core::clone::Clone for SPCFGRULEATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SPCOMMITFLAGS(pub i32);
pub const SPCF_NONE: SPCOMMITFLAGS = SPCOMMITFLAGS(0i32);
pub const SPCF_ADD_TO_USER_LEXICON: SPCOMMITFLAGS = SPCOMMITFLAGS(1i32);
pub const SPCF_DEFINITE_CORRECTION: SPCOMMITFLAGS = SPCOMMITFLAGS(2i32);
impl ::core::marker::Copy for SPCOMMITFLAGS {}
impl ::core::clone::Clone for SPCOMMITFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SPCONTEXTSTATE(pub i32);
pub const SPCS_DISABLED: SPCONTEXTSTATE = SPCONTEXTSTATE(0i32);
pub const SPCS_ENABLED: SPCONTEXTSTATE = SPCONTEXTSTATE(1i32);
impl ::core::marker::Copy for SPCONTEXTSTATE {}
impl ::core::clone::Clone for SPCONTEXTSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SPDATAKEYLOCATION(pub i32);
pub const SPDKL_DefaultLocation: SPDATAKEYLOCATION = SPDATAKEYLOCATION(0i32);
pub const SPDKL_CurrentUser: SPDATAKEYLOCATION = SPDATAKEYLOCATION(1i32);
pub const SPDKL_LocalMachine: SPDATAKEYLOCATION = SPDATAKEYLOCATION(2i32);
pub const SPDKL_CurrentConfig: SPDATAKEYLOCATION = SPDATAKEYLOCATION(5i32);
impl ::core::marker::Copy for SPDATAKEYLOCATION {}
impl ::core::clone::Clone for SPDATAKEYLOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SPDISPLYATTRIBUTES(pub i32);
pub const SPAF_ONE_TRAILING_SPACE: SPDISPLYATTRIBUTES = SPDISPLYATTRIBUTES(2i32);
pub const SPAF_TWO_TRAILING_SPACES: SPDISPLYATTRIBUTES = SPDISPLYATTRIBUTES(4i32);
pub const SPAF_CONSUME_LEADING_SPACES: SPDISPLYATTRIBUTES = SPDISPLYATTRIBUTES(8i32);
pub const SPAF_BUFFER_POSITION: SPDISPLYATTRIBUTES = SPDISPLYATTRIBUTES(16i32);
pub const SPAF_ALL: SPDISPLYATTRIBUTES = SPDISPLYATTRIBUTES(31i32);
pub const SPAF_USER_SPECIFIED: SPDISPLYATTRIBUTES = SPDISPLYATTRIBUTES(128i32);
impl ::core::marker::Copy for SPDISPLYATTRIBUTES {}
impl ::core::clone::Clone for SPDISPLYATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SPEAKFLAGS {}
impl ::core::clone::Clone for SPEAKFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SPENDSRSTREAMFLAGS(pub i32);
pub const SPESF_NONE: SPENDSRSTREAMFLAGS = SPENDSRSTREAMFLAGS(0i32);
pub const SPESF_STREAM_RELEASED: SPENDSRSTREAMFLAGS = SPENDSRSTREAMFLAGS(1i32);
pub const SPESF_EMULATED: SPENDSRSTREAMFLAGS = SPENDSRSTREAMFLAGS(2i32);
impl ::core::marker::Copy for SPENDSRSTREAMFLAGS {}
impl ::core::clone::Clone for SPENDSRSTREAMFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SPEVENTENUM {}
impl ::core::clone::Clone for SPEVENTENUM {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SPEVENTLPARAMTYPE(pub i32);
pub const SPET_LPARAM_IS_UNDEFINED: SPEVENTLPARAMTYPE = SPEVENTLPARAMTYPE(0i32);
pub const SPET_LPARAM_IS_TOKEN: SPEVENTLPARAMTYPE = SPEVENTLPARAMTYPE(1i32);
pub const SPET_LPARAM_IS_OBJECT: SPEVENTLPARAMTYPE = SPEVENTLPARAMTYPE(2i32);
pub const SPET_LPARAM_IS_POINTER: SPEVENTLPARAMTYPE = SPEVENTLPARAMTYPE(3i32);
pub const SPET_LPARAM_IS_STRING: SPEVENTLPARAMTYPE = SPEVENTLPARAMTYPE(4i32);
impl ::core::marker::Copy for SPEVENTLPARAMTYPE {}
impl ::core::clone::Clone for SPEVENTLPARAMTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SPFILEMODE(pub i32);
pub const SPFM_OPEN_READONLY: SPFILEMODE = SPFILEMODE(0i32);
pub const SPFM_OPEN_READWRITE: SPFILEMODE = SPFILEMODE(1i32);
pub const SPFM_CREATE: SPFILEMODE = SPFILEMODE(2i32);
pub const SPFM_CREATE_ALWAYS: SPFILEMODE = SPFILEMODE(3i32);
pub const SPFM_NUM_MODES: SPFILEMODE = SPFILEMODE(4i32);
impl ::core::marker::Copy for SPFILEMODE {}
impl ::core::clone::Clone for SPFILEMODE {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SPGRAMMAROPTIONS {}
impl ::core::clone::Clone for SPGRAMMAROPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SPGRAMMARSTATE(pub i32);
pub const SPGS_DISABLED: SPGRAMMARSTATE = SPGRAMMARSTATE(0i32);
pub const SPGS_ENABLED: SPGRAMMARSTATE = SPGRAMMARSTATE(1i32);
pub const SPGS_EXCLUSIVE: SPGRAMMARSTATE = SPGRAMMARSTATE(3i32);
impl ::core::marker::Copy for SPGRAMMARSTATE {}
impl ::core::clone::Clone for SPGRAMMARSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SPGRAMMARWORDTYPE(pub i32);
pub const SPWT_DISPLAY: SPGRAMMARWORDTYPE = SPGRAMMARWORDTYPE(0i32);
pub const SPWT_LEXICAL: SPGRAMMARWORDTYPE = SPGRAMMARWORDTYPE(1i32);
pub const SPWT_PRONUNCIATION: SPGRAMMARWORDTYPE = SPGRAMMARWORDTYPE(2i32);
pub const SPWT_LEXICAL_NO_SPECIAL_CHARS: SPGRAMMARWORDTYPE = SPGRAMMARWORDTYPE(3i32);
impl ::core::marker::Copy for SPGRAMMARWORDTYPE {}
impl ::core::clone::Clone for SPGRAMMARWORDTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SPINTERFERENCE {}
impl ::core::clone::Clone for SPINTERFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SPLEXICONTYPE {}
impl ::core::clone::Clone for SPLEXICONTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SPLOADOPTIONS(pub i32);
pub const SPLO_STATIC: SPLOADOPTIONS = SPLOADOPTIONS(0i32);
pub const SPLO_DYNAMIC: SPLOADOPTIONS = SPLOADOPTIONS(1i32);
impl ::core::marker::Copy for SPLOADOPTIONS {}
impl ::core::clone::Clone for SPLOADOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SPMATCHINGMODE(pub i32);
pub const AllWords: SPMATCHINGMODE = SPMATCHINGMODE(0i32);
pub const Subsequence: SPMATCHINGMODE = SPMATCHINGMODE(1i32);
pub const OrderedSubset: SPMATCHINGMODE = SPMATCHINGMODE(3i32);
pub const SubsequenceContentRequired: SPMATCHINGMODE = SPMATCHINGMODE(5i32);
pub const OrderedSubsetContentRequired: SPMATCHINGMODE = SPMATCHINGMODE(7i32);
impl ::core::marker::Copy for SPMATCHINGMODE {}
impl ::core::clone::Clone for SPMATCHINGMODE {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SPPARTOFSPEECH {}
impl ::core::clone::Clone for SPPARTOFSPEECH {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SPPHRASEPROPERTYUNIONTYPE(pub i32);
pub const SPPPUT_UNUSED: SPPHRASEPROPERTYUNIONTYPE = SPPHRASEPROPERTYUNIONTYPE(0i32);
pub const SPPPUT_ARRAY_INDEX: SPPHRASEPROPERTYUNIONTYPE = SPPHRASEPROPERTYUNIONTYPE(1i32);
impl ::core::marker::Copy for SPPHRASEPROPERTYUNIONTYPE {}
impl ::core::clone::Clone for SPPHRASEPROPERTYUNIONTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SPPHRASERNG(pub i32);
pub const SPPR_ALL_ELEMENTS: SPPHRASERNG = SPPHRASERNG(-1i32);
impl ::core::marker::Copy for SPPHRASERNG {}
impl ::core::clone::Clone for SPPHRASERNG {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SPPRONUNCIATIONFLAGS(pub i32);
pub const ePRONFLAG_USED: SPPRONUNCIATIONFLAGS = SPPRONUNCIATIONFLAGS(1i32);
impl ::core::marker::Copy for SPPRONUNCIATIONFLAGS {}
impl ::core::clone::Clone for SPPRONUNCIATIONFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SPRECOEVENTFLAGS(pub i32);
pub const SPREF_AutoPause: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(1i32);
pub const SPREF_Emulated: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(2i32);
pub const SPREF_SMLTimeout: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(4i32);
pub const SPREF_ExtendableParse: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(8i32);
pub const SPREF_ReSent: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(16i32);
pub const SPREF_Hypothesis: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(32i32);
pub const SPREF_FalseRecognition: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(64i32);
impl ::core::marker::Copy for SPRECOEVENTFLAGS {}
impl ::core::clone::Clone for SPRECOEVENTFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SPRECOSTATE(pub i32);
pub const SPRST_INACTIVE: SPRECOSTATE = SPRECOSTATE(0i32);
pub const SPRST_ACTIVE: SPRECOSTATE = SPRECOSTATE(1i32);
pub const SPRST_ACTIVE_ALWAYS: SPRECOSTATE = SPRECOSTATE(2i32);
pub const SPRST_INACTIVE_WITH_PURGE: SPRECOSTATE = SPRECOSTATE(3i32);
pub const SPRST_NUM_STATES: SPRECOSTATE = SPRECOSTATE(4i32);
impl ::core::marker::Copy for SPRECOSTATE {}
impl ::core::clone::Clone for SPRECOSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SPRULESTATE(pub i32);
pub const SPRS_INACTIVE: SPRULESTATE = SPRULESTATE(0i32);
pub const SPRS_ACTIVE: SPRULESTATE = SPRULESTATE(1i32);
pub const SPRS_ACTIVE_WITH_AUTO_PAUSE: SPRULESTATE = SPRULESTATE(3i32);
pub const SPRS_ACTIVE_USER_DELIMITED: SPRULESTATE = SPRULESTATE(4i32);
impl ::core::marker::Copy for SPRULESTATE {}
impl ::core::clone::Clone for SPRULESTATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SPRUNSTATE(pub i32);
pub const SPRS_DONE: SPRUNSTATE = SPRUNSTATE(1i32);
pub const SPRS_IS_SPEAKING: SPRUNSTATE = SPRUNSTATE(2i32);
impl ::core::marker::Copy for SPRUNSTATE {}
impl ::core::clone::Clone for SPRUNSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SPSEMANTICFORMAT(pub i32);
pub const SPSMF_SAPI_PROPERTIES: SPSEMANTICFORMAT = SPSEMANTICFORMAT(0i32);
pub const SPSMF_SRGS_SEMANTICINTERPRETATION_MS: SPSEMANTICFORMAT = SPSEMANTICFORMAT(1i32);
pub const SPSMF_SRGS_SAPIPROPERTIES: SPSEMANTICFORMAT = SPSEMANTICFORMAT(2i32);
pub const SPSMF_UPS: SPSEMANTICFORMAT = SPSEMANTICFORMAT(4i32);
pub const SPSMF_SRGS_SEMANTICINTERPRETATION_W3C: SPSEMANTICFORMAT = SPSEMANTICFORMAT(8i32);
impl ::core::marker::Copy for SPSEMANTICFORMAT {}
impl ::core::clone::Clone for SPSEMANTICFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SPSHORTCUTTYPE {}
impl ::core::clone::Clone for SPSHORTCUTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SPSTREAMFORMAT {}
impl ::core::clone::Clone for SPSTREAMFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SPVACTIONS(pub i32);
pub const SPVA_Speak: SPVACTIONS = SPVACTIONS(0i32);
pub const SPVA_Silence: SPVACTIONS = SPVACTIONS(1i32);
pub const SPVA_Pronounce: SPVACTIONS = SPVACTIONS(2i32);
pub const SPVA_Bookmark: SPVACTIONS = SPVACTIONS(3i32);
pub const SPVA_SpellOut: SPVACTIONS = SPVACTIONS(4i32);
pub const SPVA_Section: SPVACTIONS = SPVACTIONS(5i32);
pub const SPVA_ParseUnknownTag: SPVACTIONS = SPVACTIONS(6i32);
impl ::core::marker::Copy for SPVACTIONS {}
impl ::core::clone::Clone for SPVACTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SPVALUETYPE {}
impl ::core::clone::Clone for SPVALUETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SPVFEATURE(pub i32);
pub const SPVFEATURE_STRESSED: SPVFEATURE = SPVFEATURE(1i32);
pub const SPVFEATURE_EMPHASIS: SPVFEATURE = SPVFEATURE(2i32);
impl ::core::marker::Copy for SPVFEATURE {}
impl ::core::clone::Clone for SPVFEATURE {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SPVISEMES {}
impl ::core::clone::Clone for SPVISEMES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SPVLIMITS(pub i32);
pub const SPMIN_VOLUME: SPVLIMITS = SPVLIMITS(0i32);
pub const SPMAX_VOLUME: SPVLIMITS = SPVLIMITS(100i32);
pub const SPMIN_RATE: SPVLIMITS = SPVLIMITS(-10i32);
pub const SPMAX_RATE: SPVLIMITS = SPVLIMITS(10i32);
impl ::core::marker::Copy for SPVLIMITS {}
impl ::core::clone::Clone for SPVLIMITS {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SPVPRIORITY(pub i32);
pub const SPVPRI_NORMAL: SPVPRIORITY = SPVPRIORITY(0i32);
pub const SPVPRI_ALERT: SPVPRIORITY = SPVPRIORITY(1i32);
pub const SPVPRI_OVER: SPVPRIORITY = SPVPRIORITY(2i32);
impl ::core::marker::Copy for SPVPRIORITY {}
impl ::core::clone::Clone for SPVPRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SPWAVEFORMATTYPE(pub i32);
pub const SPWF_INPUT: SPWAVEFORMATTYPE = SPWAVEFORMATTYPE(0i32);
pub const SPWF_SRENGINE: SPWAVEFORMATTYPE = SPWAVEFORMATTYPE(1i32);
impl ::core::marker::Copy for SPWAVEFORMATTYPE {}
impl ::core::clone::Clone for SPWAVEFORMATTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SPWORDPRONOUNCEABLE(pub i32);
pub const SPWP_UNKNOWN_WORD_UNPRONOUNCEABLE: SPWORDPRONOUNCEABLE = SPWORDPRONOUNCEABLE(0i32);
pub const SPWP_UNKNOWN_WORD_PRONOUNCEABLE: SPWORDPRONOUNCEABLE = SPWORDPRONOUNCEABLE(1i32);
pub const SPWP_KNOWN_WORD_PRONOUNCEABLE: SPWORDPRONOUNCEABLE = SPWORDPRONOUNCEABLE(2i32);
impl ::core::marker::Copy for SPWORDPRONOUNCEABLE {}
impl ::core::clone::Clone for SPWORDPRONOUNCEABLE {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SPWORDTYPE(pub i32);
pub const eWORDTYPE_ADDED: SPWORDTYPE = SPWORDTYPE(1i32);
pub const eWORDTYPE_DELETED: SPWORDTYPE = SPWORDTYPE(2i32);
impl ::core::marker::Copy for SPWORDTYPE {}
impl ::core::clone::Clone for SPWORDTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SPXMLRESULTOPTIONS(pub i32);
pub const SPXRO_SML: SPXMLRESULTOPTIONS = SPXMLRESULTOPTIONS(0i32);
pub const SPXRO_Alternates_SML: SPXMLRESULTOPTIONS = SPXMLRESULTOPTIONS(1i32);
impl ::core::marker::Copy for SPXMLRESULTOPTIONS {}
impl ::core::clone::Clone for SPXMLRESULTOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SP_EMULATE_RESULT: u32 = 1073741824u32;
pub const SP_LOW_CONFIDENCE: i32 = -1i32;
pub const SP_MAX_LANGIDS: u32 = 20u32;
pub const SP_MAX_PRON_LENGTH: u32 = 384u32;
pub const SP_MAX_WORD_LENGTH: u32 = 128u32;
pub const SP_NORMAL_CONFIDENCE: u32 = 0u32;
pub const SP_STREAMPOS_ASAP: u32 = 0u32;
pub const SP_STREAMPOS_REALTIME: i32 = -1i32;
pub const SpAudioFormat: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2667145328, data2: 57696, data3: 18322, data4: [130, 13, 72, 207, 6, 73, 228, 236] };
pub const SpCompressedLexicon: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2425370390, data2: 12098, data3: 4563, data4: [156, 38, 0, 192, 79, 142, 248, 124] };
pub const SpCustomStream: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2378101055,
    data2: 6472,
    data3: 19112,
    data4: [140, 240, 4, 142, 235, 237, 149, 216],
};
pub const SpFileStream: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2490897075,
    data2: 10977,
    data3: 17988,
    data4: [186, 134, 158, 144, 222, 215, 236, 145],
};
pub const SpInProcRecoContext: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1940744258,
    data2: 44256,
    data3: 17896,
    data4: [164, 221, 135, 149, 136, 26, 44, 42],
};
pub const SpInprocRecognizer: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1102617451, data2: 37785, data3: 4562, data4: [150, 35, 0, 192, 79, 142, 230, 40] };
pub const SpLexicon: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 106292118, data2: 9680, data3: 4563, data4: [156, 38, 0, 192, 79, 142, 248, 124] };
pub const SpMMAudioEnum: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2870513824, data2: 59679, data3: 4562, data4: [187, 145, 0, 192, 79, 142, 230, 192] };
pub const SpMMAudioIn: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3476893264, data2: 21490, data3: 4562, data4: [150, 12, 0, 192, 79, 142, 230, 40] };
pub const SpMMAudioOut: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2831581419, data2: 15666, data3: 4562, data4: [158, 231, 0, 192, 79, 121, 115, 150] };
pub const SpMemoryStream: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1605889917,
    data2: 57332,
    data3: 18058,
    data4: [182, 183, 47, 203, 209, 136, 249, 148],
};
pub const SpNotifyTranslator: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3803075442, data2: 23872, data3: 4562, data4: [150, 14, 0, 192, 79, 142, 230, 40] };
pub const SpNullPhoneConverter: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1163863273,
    data2: 29590,
    data3: 18966,
    data4: [151, 21, 124, 15, 219, 227, 239, 227],
};
pub const SpObjectToken: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4014020434,
    data2: 14134,
    data3: 19636,
    data4: [156, 140, 142, 244, 204, 181, 142, 254],
};
pub const SpObjectTokenCategory: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2836404351,
    data2: 3194,
    data3: 17836,
    data4: [146, 204, 89, 237, 175, 183, 123, 83],
};
pub const SpPhoneConverter: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2441475907,
    data2: 4419,
    data3: 19496,
    data4: [134, 181, 191, 241, 79, 32, 229, 200],
};
pub const SpPhoneticAlphabetConverter: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1329676582,
    data2: 57315,
    data3: 17961,
    data4: [153, 238, 121, 121, 120, 49, 126, 173],
};
pub const SpPhraseInfoBuilder: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3258958477,
    data2: 50527,
    data3: 18208,
    data4: [139, 50, 145, 247, 60, 43, 213, 209],
};
pub const SpResourceManager: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2524222323, data2: 13201, data3: 4562, data4: [158, 227, 0, 192, 79, 121, 115, 150] };
pub const SpSharedRecoContext: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1193304580, data2: 24266, data3: 4562, data4: [150, 15, 0, 192, 79, 142, 230, 40] };
pub const SpSharedRecognizer: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1005471888, data2: 20457, data3: 18999, data4: [140, 30, 94, 126, 18, 121, 28, 31] };
pub const SpShortcut: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 225586970,
    data2: 40911,
    data3: 20066,
    data4: [150, 216, 109, 248, 240, 26, 38, 170],
};
pub const SpStream: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1901960281, data2: 17474, data3: 4562, data4: [150, 5, 0, 192, 79, 142, 230, 40] };
pub const SpStreamFormatConverter: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1880331322, data2: 58092, data3: 4562, data4: [160, 134, 0, 192, 79, 142, 249, 181] };
pub const SpTextSelectionInformation: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 261227274,
    data2: 52221,
    data3: 19128,
    data4: [161, 100, 255, 89, 133, 84, 127, 246],
};
pub const SpUnCompressedLexicon: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3387128853,
    data2: 57234,
    data3: 18215,
    data4: [133, 214, 114, 229, 238, 182, 153, 90],
};
pub const SpVoice: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2524222327, data2: 13201, data3: 4562, data4: [158, 227, 0, 192, 79, 121, 115, 150] };
pub const SpWaveFormatEx: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3348780876,
    data2: 25534,
    data3: 17593,
    data4: [128, 31, 40, 63, 135, 248, 152, 190],
};
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
impl ::core::marker::Copy for SpeechAudioFormatType {}
impl ::core::clone::Clone for SpeechAudioFormatType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechAudioState(pub i32);
pub const SASClosed: SpeechAudioState = SpeechAudioState(0i32);
pub const SASStop: SpeechAudioState = SpeechAudioState(1i32);
pub const SASPause: SpeechAudioState = SpeechAudioState(2i32);
pub const SASRun: SpeechAudioState = SpeechAudioState(3i32);
impl ::core::marker::Copy for SpeechAudioState {}
impl ::core::clone::Clone for SpeechAudioState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechBookmarkOptions(pub i32);
pub const SBONone: SpeechBookmarkOptions = SpeechBookmarkOptions(0i32);
pub const SBOPause: SpeechBookmarkOptions = SpeechBookmarkOptions(1i32);
impl ::core::marker::Copy for SpeechBookmarkOptions {}
impl ::core::clone::Clone for SpeechBookmarkOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechDataKeyLocation(pub i32);
pub const SDKLDefaultLocation: SpeechDataKeyLocation = SpeechDataKeyLocation(0i32);
pub const SDKLCurrentUser: SpeechDataKeyLocation = SpeechDataKeyLocation(1i32);
pub const SDKLLocalMachine: SpeechDataKeyLocation = SpeechDataKeyLocation(2i32);
pub const SDKLCurrentConfig: SpeechDataKeyLocation = SpeechDataKeyLocation(5i32);
impl ::core::marker::Copy for SpeechDataKeyLocation {}
impl ::core::clone::Clone for SpeechDataKeyLocation {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SpeechDiscardType {}
impl ::core::clone::Clone for SpeechDiscardType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechDisplayAttributes(pub i32);
pub const SDA_No_Trailing_Space: SpeechDisplayAttributes = SpeechDisplayAttributes(0i32);
pub const SDA_One_Trailing_Space: SpeechDisplayAttributes = SpeechDisplayAttributes(2i32);
pub const SDA_Two_Trailing_Spaces: SpeechDisplayAttributes = SpeechDisplayAttributes(4i32);
pub const SDA_Consume_Leading_Spaces: SpeechDisplayAttributes = SpeechDisplayAttributes(8i32);
impl ::core::marker::Copy for SpeechDisplayAttributes {}
impl ::core::clone::Clone for SpeechDisplayAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechEmulationCompareFlags(pub i32);
pub const SECFIgnoreCase: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(1i32);
pub const SECFIgnoreKanaType: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(65536i32);
pub const SECFIgnoreWidth: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(131072i32);
pub const SECFNoSpecialChars: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(536870912i32);
pub const SECFEmulateResult: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(1073741824i32);
pub const SECFDefault: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(196609i32);
impl ::core::marker::Copy for SpeechEmulationCompareFlags {}
impl ::core::clone::Clone for SpeechEmulationCompareFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechEngineConfidence(pub i32);
pub const SECLowConfidence: SpeechEngineConfidence = SpeechEngineConfidence(-1i32);
pub const SECNormalConfidence: SpeechEngineConfidence = SpeechEngineConfidence(0i32);
pub const SECHighConfidence: SpeechEngineConfidence = SpeechEngineConfidence(1i32);
impl ::core::marker::Copy for SpeechEngineConfidence {}
impl ::core::clone::Clone for SpeechEngineConfidence {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechFormatType(pub i32);
pub const SFTInput: SpeechFormatType = SpeechFormatType(0i32);
pub const SFTSREngine: SpeechFormatType = SpeechFormatType(1i32);
impl ::core::marker::Copy for SpeechFormatType {}
impl ::core::clone::Clone for SpeechFormatType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechGrammarRuleStateTransitionType(pub i32);
pub const SGRSTTEpsilon: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(0i32);
pub const SGRSTTWord: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(1i32);
pub const SGRSTTRule: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(2i32);
pub const SGRSTTDictation: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(3i32);
pub const SGRSTTWildcard: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(4i32);
pub const SGRSTTTextBuffer: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(5i32);
impl ::core::marker::Copy for SpeechGrammarRuleStateTransitionType {}
impl ::core::clone::Clone for SpeechGrammarRuleStateTransitionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechGrammarState(pub i32);
pub const SGSEnabled: SpeechGrammarState = SpeechGrammarState(1i32);
pub const SGSDisabled: SpeechGrammarState = SpeechGrammarState(0i32);
pub const SGSExclusive: SpeechGrammarState = SpeechGrammarState(3i32);
impl ::core::marker::Copy for SpeechGrammarState {}
impl ::core::clone::Clone for SpeechGrammarState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechGrammarWordType(pub i32);
pub const SGDisplay: SpeechGrammarWordType = SpeechGrammarWordType(0i32);
pub const SGLexical: SpeechGrammarWordType = SpeechGrammarWordType(1i32);
pub const SGPronounciation: SpeechGrammarWordType = SpeechGrammarWordType(2i32);
pub const SGLexicalNoSpecialChars: SpeechGrammarWordType = SpeechGrammarWordType(3i32);
impl ::core::marker::Copy for SpeechGrammarWordType {}
impl ::core::clone::Clone for SpeechGrammarWordType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechInterference(pub i32);
pub const SINone: SpeechInterference = SpeechInterference(0i32);
pub const SINoise: SpeechInterference = SpeechInterference(1i32);
pub const SINoSignal: SpeechInterference = SpeechInterference(2i32);
pub const SITooLoud: SpeechInterference = SpeechInterference(3i32);
pub const SITooQuiet: SpeechInterference = SpeechInterference(4i32);
pub const SITooFast: SpeechInterference = SpeechInterference(5i32);
pub const SITooSlow: SpeechInterference = SpeechInterference(6i32);
impl ::core::marker::Copy for SpeechInterference {}
impl ::core::clone::Clone for SpeechInterference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechLexiconType(pub i32);
pub const SLTUser: SpeechLexiconType = SpeechLexiconType(1i32);
pub const SLTApp: SpeechLexiconType = SpeechLexiconType(2i32);
impl ::core::marker::Copy for SpeechLexiconType {}
impl ::core::clone::Clone for SpeechLexiconType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechLoadOption(pub i32);
pub const SLOStatic: SpeechLoadOption = SpeechLoadOption(0i32);
pub const SLODynamic: SpeechLoadOption = SpeechLoadOption(1i32);
impl ::core::marker::Copy for SpeechLoadOption {}
impl ::core::clone::Clone for SpeechLoadOption {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SpeechPartOfSpeech {}
impl ::core::clone::Clone for SpeechPartOfSpeech {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecoContextState(pub i32);
pub const SRCS_Disabled: SpeechRecoContextState = SpeechRecoContextState(0i32);
pub const SRCS_Enabled: SpeechRecoContextState = SpeechRecoContextState(1i32);
impl ::core::marker::Copy for SpeechRecoContextState {}
impl ::core::clone::Clone for SpeechRecoContextState {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SpeechRecoEvents {}
impl ::core::clone::Clone for SpeechRecoEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognitionType(pub i32);
pub const SRTStandard: SpeechRecognitionType = SpeechRecognitionType(0i32);
pub const SRTAutopause: SpeechRecognitionType = SpeechRecognitionType(1i32);
pub const SRTEmulated: SpeechRecognitionType = SpeechRecognitionType(2i32);
pub const SRTSMLTimeout: SpeechRecognitionType = SpeechRecognitionType(4i32);
pub const SRTExtendableParse: SpeechRecognitionType = SpeechRecognitionType(8i32);
pub const SRTReSent: SpeechRecognitionType = SpeechRecognitionType(16i32);
impl ::core::marker::Copy for SpeechRecognitionType {}
impl ::core::clone::Clone for SpeechRecognitionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognizerState(pub i32);
pub const SRSInactive: SpeechRecognizerState = SpeechRecognizerState(0i32);
pub const SRSActive: SpeechRecognizerState = SpeechRecognizerState(1i32);
pub const SRSActiveAlways: SpeechRecognizerState = SpeechRecognizerState(2i32);
pub const SRSInactiveWithPurge: SpeechRecognizerState = SpeechRecognizerState(3i32);
impl ::core::marker::Copy for SpeechRecognizerState {}
impl ::core::clone::Clone for SpeechRecognizerState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRetainedAudioOptions(pub i32);
pub const SRAONone: SpeechRetainedAudioOptions = SpeechRetainedAudioOptions(0i32);
pub const SRAORetainAudio: SpeechRetainedAudioOptions = SpeechRetainedAudioOptions(1i32);
impl ::core::marker::Copy for SpeechRetainedAudioOptions {}
impl ::core::clone::Clone for SpeechRetainedAudioOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRuleAttributes(pub i32);
pub const SRATopLevel: SpeechRuleAttributes = SpeechRuleAttributes(1i32);
pub const SRADefaultToActive: SpeechRuleAttributes = SpeechRuleAttributes(2i32);
pub const SRAExport: SpeechRuleAttributes = SpeechRuleAttributes(4i32);
pub const SRAImport: SpeechRuleAttributes = SpeechRuleAttributes(8i32);
pub const SRAInterpreter: SpeechRuleAttributes = SpeechRuleAttributes(16i32);
pub const SRADynamic: SpeechRuleAttributes = SpeechRuleAttributes(32i32);
pub const SRARoot: SpeechRuleAttributes = SpeechRuleAttributes(64i32);
impl ::core::marker::Copy for SpeechRuleAttributes {}
impl ::core::clone::Clone for SpeechRuleAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRuleState(pub i32);
pub const SGDSInactive: SpeechRuleState = SpeechRuleState(0i32);
pub const SGDSActive: SpeechRuleState = SpeechRuleState(1i32);
pub const SGDSActiveWithAutoPause: SpeechRuleState = SpeechRuleState(3i32);
pub const SGDSActiveUserDelimited: SpeechRuleState = SpeechRuleState(4i32);
impl ::core::marker::Copy for SpeechRuleState {}
impl ::core::clone::Clone for SpeechRuleState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRunState(pub i32);
pub const SRSEDone: SpeechRunState = SpeechRunState(1i32);
pub const SRSEIsSpeaking: SpeechRunState = SpeechRunState(2i32);
impl ::core::marker::Copy for SpeechRunState {}
impl ::core::clone::Clone for SpeechRunState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechSpecialTransitionType(pub i32);
pub const SSTTWildcard: SpeechSpecialTransitionType = SpeechSpecialTransitionType(1i32);
pub const SSTTDictation: SpeechSpecialTransitionType = SpeechSpecialTransitionType(2i32);
pub const SSTTTextBuffer: SpeechSpecialTransitionType = SpeechSpecialTransitionType(3i32);
impl ::core::marker::Copy for SpeechSpecialTransitionType {}
impl ::core::clone::Clone for SpeechSpecialTransitionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechStreamFileMode(pub i32);
pub const SSFMOpenForRead: SpeechStreamFileMode = SpeechStreamFileMode(0i32);
pub const SSFMOpenReadWrite: SpeechStreamFileMode = SpeechStreamFileMode(1i32);
pub const SSFMCreate: SpeechStreamFileMode = SpeechStreamFileMode(2i32);
pub const SSFMCreateForWrite: SpeechStreamFileMode = SpeechStreamFileMode(3i32);
impl ::core::marker::Copy for SpeechStreamFileMode {}
impl ::core::clone::Clone for SpeechStreamFileMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechStreamSeekPositionType(pub u32);
pub const SSSPTRelativeToStart: SpeechStreamSeekPositionType = SpeechStreamSeekPositionType(0u32);
pub const SSSPTRelativeToCurrentPosition: SpeechStreamSeekPositionType = SpeechStreamSeekPositionType(1u32);
pub const SSSPTRelativeToEnd: SpeechStreamSeekPositionType = SpeechStreamSeekPositionType(2u32);
impl ::core::marker::Copy for SpeechStreamSeekPositionType {}
impl ::core::clone::Clone for SpeechStreamSeekPositionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechTokenContext(pub u32);
pub const STCInprocServer: SpeechTokenContext = SpeechTokenContext(1u32);
pub const STCInprocHandler: SpeechTokenContext = SpeechTokenContext(2u32);
pub const STCLocalServer: SpeechTokenContext = SpeechTokenContext(4u32);
pub const STCRemoteServer: SpeechTokenContext = SpeechTokenContext(16u32);
pub const STCAll: SpeechTokenContext = SpeechTokenContext(23u32);
impl ::core::marker::Copy for SpeechTokenContext {}
impl ::core::clone::Clone for SpeechTokenContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechTokenShellFolder(pub i32);
pub const STSF_AppData: SpeechTokenShellFolder = SpeechTokenShellFolder(26i32);
pub const STSF_LocalAppData: SpeechTokenShellFolder = SpeechTokenShellFolder(28i32);
pub const STSF_CommonAppData: SpeechTokenShellFolder = SpeechTokenShellFolder(35i32);
pub const STSF_FlagCreate: SpeechTokenShellFolder = SpeechTokenShellFolder(32768i32);
impl ::core::marker::Copy for SpeechTokenShellFolder {}
impl ::core::clone::Clone for SpeechTokenShellFolder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechVisemeFeature(pub i32);
pub const SVF_None: SpeechVisemeFeature = SpeechVisemeFeature(0i32);
pub const SVF_Stressed: SpeechVisemeFeature = SpeechVisemeFeature(1i32);
pub const SVF_Emphasis: SpeechVisemeFeature = SpeechVisemeFeature(2i32);
impl ::core::marker::Copy for SpeechVisemeFeature {}
impl ::core::clone::Clone for SpeechVisemeFeature {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SpeechVisemeType {}
impl ::core::clone::Clone for SpeechVisemeType {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SpeechVoiceEvents {}
impl ::core::clone::Clone for SpeechVoiceEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechVoicePriority(pub i32);
pub const SVPNormal: SpeechVoicePriority = SpeechVoicePriority(0i32);
pub const SVPAlert: SpeechVoicePriority = SpeechVoicePriority(1i32);
pub const SVPOver: SpeechVoicePriority = SpeechVoicePriority(2i32);
impl ::core::marker::Copy for SpeechVoicePriority {}
impl ::core::clone::Clone for SpeechVoicePriority {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SpeechVoiceSpeakFlags {}
impl ::core::clone::Clone for SpeechVoiceSpeakFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechWordPronounceable(pub i32);
pub const SWPUnknownWordUnpronounceable: SpeechWordPronounceable = SpeechWordPronounceable(0i32);
pub const SWPUnknownWordPronounceable: SpeechWordPronounceable = SpeechWordPronounceable(1i32);
pub const SWPKnownWordPronounceable: SpeechWordPronounceable = SpeechWordPronounceable(2i32);
impl ::core::marker::Copy for SpeechWordPronounceable {}
impl ::core::clone::Clone for SpeechWordPronounceable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechWordType(pub i32);
pub const SWTAdded: SpeechWordType = SpeechWordType(1i32);
pub const SWTDeleted: SpeechWordType = SpeechWordType(2i32);
impl ::core::marker::Copy for SpeechWordType {}
impl ::core::clone::Clone for SpeechWordType {
    fn clone(&self) -> Self {
        *self
    }
}
pub const Speech_Default_Weight: f32 = 1f32;
pub const Speech_Max_Pron_Length: i32 = 384i32;
pub const Speech_Max_Word_Length: i32 = 128i32;
pub const Speech_StreamPos_Asap: i32 = 0i32;
pub const Speech_StreamPos_RealTime: i32 = -1i32;
#[repr(transparent)]
pub struct _ISpeechRecoContextEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct _ISpeechVoiceEvents(pub *mut ::core::ffi::c_void);
