#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[doc = "*Required features: `Win32_Media_Speech`*"]
pub const DEFAULT_WEIGHT: u32 = 1u32;
pub struct DISPIDSPRG(i32);
pub struct DISPIDSPTSI(i32);
pub struct DISPID_SpeechAudio(i32);
pub struct DISPID_SpeechAudioBufferInfo(i32);
pub struct DISPID_SpeechAudioFormat(i32);
pub struct DISPID_SpeechAudioStatus(i32);
pub struct DISPID_SpeechBaseStream(i32);
pub struct DISPID_SpeechCustomStream(i32);
pub struct DISPID_SpeechDataKey(i32);
pub struct DISPID_SpeechFileStream(i32);
pub struct DISPID_SpeechGrammarRule(i32);
pub struct DISPID_SpeechGrammarRuleState(i32);
pub struct DISPID_SpeechGrammarRuleStateTransition(i32);
pub struct DISPID_SpeechGrammarRuleStateTransitions(i32);
pub struct DISPID_SpeechGrammarRules(i32);
pub struct DISPID_SpeechLexicon(i32);
pub struct DISPID_SpeechLexiconProns(i32);
pub struct DISPID_SpeechLexiconPronunciation(i32);
pub struct DISPID_SpeechLexiconWord(i32);
pub struct DISPID_SpeechLexiconWords(i32);
pub struct DISPID_SpeechMMSysAudio(i32);
pub struct DISPID_SpeechMemoryStream(i32);
pub struct DISPID_SpeechObjectToken(i32);
pub struct DISPID_SpeechObjectTokenCategory(i32);
pub struct DISPID_SpeechObjectTokens(i32);
pub struct DISPID_SpeechPhoneConverter(i32);
pub struct DISPID_SpeechPhraseAlternate(i32);
pub struct DISPID_SpeechPhraseAlternates(i32);
pub struct DISPID_SpeechPhraseBuilder(i32);
pub struct DISPID_SpeechPhraseElement(i32);
pub struct DISPID_SpeechPhraseElements(i32);
pub struct DISPID_SpeechPhraseInfo(i32);
pub struct DISPID_SpeechPhraseProperties(i32);
pub struct DISPID_SpeechPhraseProperty(i32);
pub struct DISPID_SpeechPhraseReplacement(i32);
pub struct DISPID_SpeechPhraseReplacements(i32);
pub struct DISPID_SpeechPhraseRule(i32);
pub struct DISPID_SpeechPhraseRules(i32);
pub struct DISPID_SpeechRecoContext(i32);
pub struct DISPID_SpeechRecoContextEvents(i32);
pub struct DISPID_SpeechRecoResult(i32);
pub struct DISPID_SpeechRecoResult2(i32);
pub struct DISPID_SpeechRecoResultTimes(i32);
pub struct DISPID_SpeechRecognizer(i32);
pub struct DISPID_SpeechRecognizerStatus(i32);
pub struct DISPID_SpeechVoice(i32);
pub struct DISPID_SpeechVoiceEvent(i32);
pub struct DISPID_SpeechVoiceStatus(i32);
pub struct DISPID_SpeechWaveFormatEx(i32);
pub struct DISPID_SpeechXMLRecoResult(i32);
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
pub struct PHONETICALPHABET(i32);
#[doc = "*Required features: `Win32_Media_Speech`*"]
pub const SAPI_ERROR_BASE: u32 = 20480u32;
pub struct SPADAPTATIONRELEVANCE(i32);
pub struct SPADAPTATIONSETTINGS(i32);
pub struct SPAUDIOBUFFERINFO(i32);
pub struct SPAUDIOOPTIONS(i32);
pub struct SPAUDIOSTATE(i32);
pub struct SPAUDIOSTATUS(i32);
pub struct SPBINARYGRAMMAR(i32);
pub struct SPBOOKMARKOPTIONS(i32);
pub struct SPCFGRULEATTRIBUTES(i32);
pub struct SPCOMMITFLAGS(i32);
pub struct SPCONTEXTSTATE(i32);
pub struct SPDATAKEYLOCATION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SPDISPLAYPHRASE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SPDISPLAYTOKEN(i32);
pub struct SPDISPLYATTRIBUTES(i32);
pub struct SPEAKFLAGS(i32);
pub struct SPENDSRSTREAMFLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SPEVENT(i32);
pub struct SPEVENTENUM(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SPEVENTEX(i32);
pub struct SPEVENTLPARAMTYPE(i32);
pub struct SPEVENTSOURCEINFO(i32);
pub struct SPFILEMODE(i32);
pub struct SPGRAMMAROPTIONS(i32);
pub struct SPGRAMMARSTATE(i32);
pub struct SPGRAMMARWORDTYPE(i32);
pub struct SPINTERFERENCE(i32);
pub struct SPLEXICONTYPE(i32);
pub struct SPLOADOPTIONS(i32);
pub struct SPMATCHINGMODE(i32);
pub struct SPNORMALIZATIONLIST(i32);
pub struct SPNOTIFYCALLBACK(i32);
pub struct SPPARTOFSPEECH(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct SPPHRASE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SPPHRASEELEMENT(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct SPPHRASEPROPERTY(i32);
pub struct SPPHRASEPROPERTYUNIONTYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SPPHRASEREPLACEMENT(i32);
pub struct SPPHRASERNG(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SPPHRASERULE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct SPPHRASE_50(i32);
pub struct SPPRONUNCIATIONFLAGS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct SPPROPERTYINFO(i32);
pub struct SPRECOCONTEXTSTATUS(i32);
pub struct SPRECOEVENTFLAGS(i32);
pub struct SPRECOGNIZERSTATUS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SPRECORESULTTIMES(i32);
pub struct SPRECOSTATE(i32);
#[doc = "*Required features: `Win32_Media_Speech`*"]
pub const SPRP_NORMAL: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
pub struct SPRULE(i32);
pub struct SPRULESTATE(i32);
pub struct SPRUNSTATE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SPSEMANTICERRORINFO(i32);
pub struct SPSEMANTICFORMAT(i32);
pub struct SPSERIALIZEDEVENT(i32);
pub struct SPSERIALIZEDEVENT64(i32);
pub struct SPSERIALIZEDPHRASE(i32);
pub struct SPSERIALIZEDRESULT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SPSHORTCUTPAIR(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SPSHORTCUTPAIRLIST(i32);
pub struct SPSHORTCUTTYPE(i32);
pub struct SPSTATEHANDLE__(i32);
pub struct SPSTREAMFORMAT(i32);
pub struct SPTEXTSELECTIONINFO(i32);
pub struct SPVACTIONS(i32);
pub struct SPVALUETYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SPVCONTEXT(i32);
pub struct SPVFEATURE(i32);
pub struct SPVISEMES(i32);
pub struct SPVLIMITS(i32);
pub struct SPVOICESTATUS(i32);
pub struct SPVPITCH(i32);
pub struct SPVPRIORITY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SPVSTATE(i32);
pub struct SPWAVEFORMATTYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SPWORD(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SPWORDLIST(i32);
pub struct SPWORDPRONOUNCEABLE(i32);
pub struct SPWORDPRONUNCIATION(i32);
pub struct SPWORDPRONUNCIATIONLIST(i32);
pub struct SPWORDTYPE(i32);
pub struct SPXMLRESULTOPTIONS(i32);
#[doc = "*Required features: `Win32_Media_Speech`*"]
pub const SP_EMULATE_RESULT: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Media_Speech`*"]
pub const SP_LOW_CONFIDENCE: i32 = -1i32;
#[doc = "*Required features: `Win32_Media_Speech`*"]
pub const SP_MAX_LANGIDS: u32 = 20u32;
#[doc = "*Required features: `Win32_Media_Speech`*"]
pub const SP_MAX_PRON_LENGTH: u32 = 384u32;
#[doc = "*Required features: `Win32_Media_Speech`*"]
pub const SP_MAX_WORD_LENGTH: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Speech`*"]
pub const SP_NORMAL_CONFIDENCE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Speech`*"]
pub const SP_STREAMPOS_ASAP: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Speech`*"]
pub const SP_STREAMPOS_REALTIME: i32 = -1i32;
pub struct SpAudioFormat(i32);
pub struct SpCompressedLexicon(i32);
pub struct SpCustomStream(i32);
pub struct SpFileStream(i32);
pub struct SpInProcRecoContext(i32);
pub struct SpInprocRecognizer(i32);
pub struct SpLexicon(i32);
pub struct SpMMAudioEnum(i32);
pub struct SpMMAudioIn(i32);
pub struct SpMMAudioOut(i32);
pub struct SpMemoryStream(i32);
pub struct SpNotifyTranslator(i32);
pub struct SpNullPhoneConverter(i32);
pub struct SpObjectToken(i32);
pub struct SpObjectTokenCategory(i32);
pub struct SpPhoneConverter(i32);
pub struct SpPhoneticAlphabetConverter(i32);
pub struct SpPhraseInfoBuilder(i32);
pub struct SpResourceManager(i32);
pub struct SpSharedRecoContext(i32);
pub struct SpSharedRecognizer(i32);
pub struct SpShortcut(i32);
pub struct SpStream(i32);
pub struct SpStreamFormatConverter(i32);
pub struct SpTextSelectionInformation(i32);
pub struct SpUnCompressedLexicon(i32);
pub struct SpVoice(i32);
pub struct SpWaveFormatEx(i32);
#[doc = "*Required features: `Win32_Media_Speech`*"]
pub const SpeechAllElements: i32 = -1i32;
pub struct SpeechAudioFormatType(i32);
pub struct SpeechAudioState(i32);
pub struct SpeechBookmarkOptions(i32);
pub struct SpeechDataKeyLocation(i32);
pub struct SpeechDiscardType(i32);
pub struct SpeechDisplayAttributes(i32);
pub struct SpeechEmulationCompareFlags(i32);
pub struct SpeechEngineConfidence(i32);
pub struct SpeechFormatType(i32);
pub struct SpeechGrammarRuleStateTransitionType(i32);
pub struct SpeechGrammarState(i32);
pub struct SpeechGrammarWordType(i32);
pub struct SpeechInterference(i32);
pub struct SpeechLexiconType(i32);
pub struct SpeechLoadOption(i32);
pub struct SpeechPartOfSpeech(i32);
pub struct SpeechRecoContextState(i32);
pub struct SpeechRecoEvents(i32);
pub struct SpeechRecognitionType(i32);
pub struct SpeechRecognizerState(i32);
pub struct SpeechRetainedAudioOptions(i32);
pub struct SpeechRuleAttributes(i32);
pub struct SpeechRuleState(i32);
pub struct SpeechRunState(i32);
pub struct SpeechSpecialTransitionType(i32);
pub struct SpeechStreamFileMode(i32);
pub struct SpeechStreamSeekPositionType(i32);
pub struct SpeechTokenContext(i32);
pub struct SpeechTokenShellFolder(i32);
pub struct SpeechVisemeFeature(i32);
pub struct SpeechVisemeType(i32);
pub struct SpeechVoiceEvents(i32);
pub struct SpeechVoicePriority(i32);
pub struct SpeechVoiceSpeakFlags(i32);
pub struct SpeechWordPronounceable(i32);
pub struct SpeechWordType(i32);
#[doc = "*Required features: `Win32_Media_Speech`*"]
pub const Speech_Default_Weight: f32 = 1f32;
#[doc = "*Required features: `Win32_Media_Speech`*"]
pub const Speech_Max_Pron_Length: i32 = 384i32;
#[doc = "*Required features: `Win32_Media_Speech`*"]
pub const Speech_Max_Word_Length: i32 = 128i32;
#[doc = "*Required features: `Win32_Media_Speech`*"]
pub const Speech_StreamPos_Asap: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Speech`*"]
pub const Speech_StreamPos_RealTime: i32 = -1i32;
#[repr(transparent)]
pub struct _ISpeechRecoContextEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct _ISpeechVoiceEvents(pub *mut ::core::ffi::c_void);
