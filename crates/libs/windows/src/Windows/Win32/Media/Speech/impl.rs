pub trait IEnumSpObjectTokensImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn Item();
    fn GetCount();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpAudioImpl: Sized + ISpStreamFormatImpl + IStreamImpl + ISequentialStreamImpl {
    fn SetState();
    fn SetFormat();
    fn GetStatus();
    fn SetBufferInfo();
    fn GetBufferInfo();
    fn GetDefaultFormat();
    fn EventHandle();
    fn GetVolumeLevel();
    fn SetVolumeLevel();
    fn GetBufferNotifySize();
    fn SetBufferNotifySize();
}
pub trait ISpContainerLexiconImpl: Sized + ISpLexiconImpl {
    fn AddLexicon();
}
pub trait ISpDataKeyImpl: Sized {
    fn SetData();
    fn GetData();
    fn SetStringValue();
    fn GetStringValue();
    fn SetDWORD();
    fn GetDWORD();
    fn OpenKey();
    fn CreateKey();
    fn DeleteKey();
    fn DeleteValue();
    fn EnumKeys();
    fn EnumValues();
}
pub trait ISpDisplayAlternatesImpl: Sized {
    fn GetDisplayAlternates();
    fn SetFullStopTrailSpace();
}
pub trait ISpEnginePronunciationImpl: Sized {
    fn Normalize();
    fn GetPronunciations();
}
pub trait ISpEventSinkImpl: Sized {
    fn AddEvents();
    fn GetEventInterest();
}
pub trait ISpEventSourceImpl: Sized + ISpNotifySourceImpl {
    fn SetInterest();
    fn GetEvents();
    fn GetInfo();
}
pub trait ISpEventSource2Impl: Sized + ISpEventSourceImpl + ISpNotifySourceImpl {
    fn GetEventsEx();
}
pub trait ISpGrammarBuilderImpl: Sized {
    fn ResetGrammar();
    fn GetRule();
    fn ClearRule();
    fn CreateNewState();
    fn AddWordTransition();
    fn AddRuleTransition();
    fn AddResource();
    fn Commit();
}
pub trait ISpGrammarBuilder2Impl: Sized {
    fn AddTextSubset();
    fn SetPhoneticAlphabet();
}
pub trait ISpLexiconImpl: Sized {
    fn GetPronunciations();
    fn AddPronunciation();
    fn RemovePronunciation();
    fn GetGeneration();
    fn GetGenerationChange();
    fn GetWords();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpMMSysAudioImpl: Sized + ISpAudioImpl + ISpStreamFormatImpl + IStreamImpl + ISequentialStreamImpl {
    fn GetDeviceId();
    fn SetDeviceId();
    fn GetMMHandle();
    fn GetLineId();
    fn SetLineId();
}
pub trait ISpNotifyCallbackImpl: Sized {
    fn NotifyCallback();
}
pub trait ISpNotifySinkImpl: Sized {
    fn Notify();
}
pub trait ISpNotifySourceImpl: Sized {
    fn SetNotifySink();
    fn SetNotifyWindowMessage();
    fn SetNotifyCallbackFunction();
    fn SetNotifyCallbackInterface();
    fn SetNotifyWin32Event();
    fn WaitForNotifyEvent();
    fn GetNotifyEventHandle();
}
pub trait ISpNotifyTranslatorImpl: Sized + ISpNotifySinkImpl {
    fn InitWindowMessage();
    fn InitCallback();
    fn InitSpNotifyCallback();
    fn InitWin32Event();
    fn Wait();
    fn GetEventHandle();
}
pub trait ISpObjectTokenImpl: Sized + ISpDataKeyImpl {
    fn SetId();
    fn GetId();
    fn GetCategory();
    fn CreateInstance();
    fn GetStorageFileName();
    fn RemoveStorageFileName();
    fn Remove();
    fn IsUISupported();
    fn DisplayUI();
    fn MatchesAttributes();
}
pub trait ISpObjectTokenCategoryImpl: Sized + ISpDataKeyImpl {
    fn SetId();
    fn GetId();
    fn GetDataKey();
    fn EnumTokens();
    fn SetDefaultTokenId();
    fn GetDefaultTokenId();
}
pub trait ISpObjectTokenInitImpl: Sized + ISpObjectTokenImpl + ISpDataKeyImpl {
    fn InitFromDataKey();
}
pub trait ISpObjectWithTokenImpl: Sized {
    fn SetObjectToken();
    fn GetObjectToken();
}
pub trait ISpPhoneConverterImpl: Sized + ISpObjectWithTokenImpl {
    fn PhoneToId();
    fn IdToPhone();
}
pub trait ISpPhoneticAlphabetConverterImpl: Sized {
    fn GetLangId();
    fn SetLangId();
    fn SAPI2UPS();
    fn UPS2SAPI();
    fn GetMaxConvertLength();
}
pub trait ISpPhoneticAlphabetSelectionImpl: Sized {
    fn IsAlphabetUPS();
    fn SetAlphabetToUPS();
}
pub trait ISpPhraseImpl: Sized {
    fn GetPhrase();
    fn GetSerializedPhrase();
    fn GetText();
    fn Discard();
}
pub trait ISpPhrase2Impl: Sized + ISpPhraseImpl {
    fn GetXMLResult();
    fn GetXMLErrorInfo();
    fn GetAudio();
}
pub trait ISpPhraseAltImpl: Sized + ISpPhraseImpl {
    fn GetAltInfo();
    fn Commit();
}
pub trait ISpPropertiesImpl: Sized {
    fn SetPropertyNum();
    fn GetPropertyNum();
    fn SetPropertyString();
    fn GetPropertyString();
}
pub trait ISpRecoContextImpl: Sized + ISpEventSourceImpl + ISpNotifySourceImpl {
    fn GetRecognizer();
    fn CreateGrammar();
    fn GetStatus();
    fn GetMaxAlternates();
    fn SetMaxAlternates();
    fn SetAudioOptions();
    fn GetAudioOptions();
    fn DeserializeResult();
    fn Bookmark();
    fn SetAdaptationData();
    fn Pause();
    fn Resume();
    fn SetVoice();
    fn GetVoice();
    fn SetVoicePurgeEvent();
    fn GetVoicePurgeEvent();
    fn SetContextState();
    fn GetContextState();
}
pub trait ISpRecoContext2Impl: Sized {
    fn SetGrammarOptions();
    fn GetGrammarOptions();
    fn SetAdaptationData2();
}
pub trait ISpRecoGrammarImpl: Sized + ISpGrammarBuilderImpl {
    fn GetGrammarId();
    fn GetRecoContext();
    fn LoadCmdFromFile();
    fn LoadCmdFromObject();
    fn LoadCmdFromResource();
    fn LoadCmdFromMemory();
    fn LoadCmdFromProprietaryGrammar();
    fn SetRuleState();
    fn SetRuleIdState();
    fn LoadDictation();
    fn UnloadDictation();
    fn SetDictationState();
    fn SetWordSequenceData();
    fn SetTextSelection();
    fn IsPronounceable();
    fn SetGrammarState();
    fn SaveCmd();
    fn GetGrammarState();
}
pub trait ISpRecoGrammar2Impl: Sized {
    fn GetRules();
    fn LoadCmdFromFile2();
    fn LoadCmdFromMemory2();
    fn SetRulePriority();
    fn SetRuleWeight();
    fn SetDictationWeight();
    fn SetGrammarLoader();
    fn SetSMLSecurityManager();
}
pub trait ISpRecoResultImpl: Sized + ISpPhraseImpl {
    fn GetResultTimes();
    fn GetAlternates();
    fn GetAudio();
    fn SpeakAudio();
    fn Serialize();
    fn ScaleAudio();
    fn GetRecoContext();
}
pub trait ISpRecoResult2Impl: Sized + ISpRecoResultImpl + ISpPhraseImpl {
    fn CommitAlternate();
    fn CommitText();
    fn SetTextFeedback();
}
pub trait ISpRecognizerImpl: Sized + ISpPropertiesImpl {
    fn SetRecognizer();
    fn GetRecognizer();
    fn SetInput();
    fn GetInputObjectToken();
    fn GetInputStream();
    fn CreateRecoContext();
    fn GetRecoProfile();
    fn SetRecoProfile();
    fn IsSharedInstance();
    fn GetRecoState();
    fn SetRecoState();
    fn GetStatus();
    fn GetFormat();
    fn IsUISupported();
    fn DisplayUI();
    fn EmulateRecognition();
}
pub trait ISpRecognizer2Impl: Sized {
    fn EmulateRecognitionEx();
    fn SetTrainingState();
    fn ResetAcousticModelAdaptation();
}
pub trait ISpRegDataKeyImpl: Sized + ISpDataKeyImpl {
    fn SetKey();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpResourceManagerImpl: Sized + IServiceProviderImpl {
    fn SetObject();
    fn GetObject();
}
pub trait ISpSerializeStateImpl: Sized {
    fn GetSerializedState();
    fn SetSerializedState();
}
pub trait ISpShortcutImpl: Sized {
    fn AddShortcut();
    fn RemoveShortcut();
    fn GetShortcuts();
    fn GetGeneration();
    fn GetWordsFromGenerationChange();
    fn GetWords();
    fn GetShortcutsForGeneration();
    fn GetGenerationChange();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpStreamImpl: Sized + ISpStreamFormatImpl + IStreamImpl + ISequentialStreamImpl {
    fn SetBaseStream();
    fn GetBaseStream();
    fn BindToFile();
    fn Close();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpStreamFormatImpl: Sized + IStreamImpl + ISequentialStreamImpl {
    fn GetFormat();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpStreamFormatConverterImpl: Sized + ISpStreamFormatImpl + IStreamImpl + ISequentialStreamImpl {
    fn SetBaseStream();
    fn GetBaseStream();
    fn SetFormat();
    fn ResetSeekPosition();
    fn ScaleConvertedToBaseOffset();
    fn ScaleBaseToConvertedOffset();
}
pub trait ISpTranscriptImpl: Sized {
    fn GetTranscript();
    fn AppendTranscript();
}
pub trait ISpVoiceImpl: Sized + ISpEventSourceImpl + ISpNotifySourceImpl {
    fn SetOutput();
    fn GetOutputObjectToken();
    fn GetOutputStream();
    fn Pause();
    fn Resume();
    fn SetVoice();
    fn GetVoice();
    fn Speak();
    fn SpeakStream();
    fn GetStatus();
    fn Skip();
    fn SetPriority();
    fn GetPriority();
    fn SetAlertBoundary();
    fn GetAlertBoundary();
    fn SetRate();
    fn GetRate();
    fn SetVolume();
    fn GetVolume();
    fn WaitUntilDone();
    fn SetSyncSpeakTimeout();
    fn GetSyncSpeakTimeout();
    fn SpeakCompleteEvent();
    fn IsUISupported();
    fn DisplayUI();
}
pub trait ISpXMLRecoResultImpl: Sized + ISpRecoResultImpl + ISpPhraseImpl {
    fn GetXMLResult();
    fn GetXMLErrorInfo();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechAudioImpl: Sized + ISpeechBaseStreamImpl + IDispatchImpl {
    fn Status();
    fn BufferInfo();
    fn DefaultFormat();
    fn Volume();
    fn SetVolume();
    fn BufferNotifySize();
    fn SetBufferNotifySize();
    fn EventHandle();
    fn SetState();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechAudioBufferInfoImpl: Sized + IDispatchImpl {
    fn MinNotification();
    fn SetMinNotification();
    fn BufferSize();
    fn SetBufferSize();
    fn EventBias();
    fn SetEventBias();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechAudioFormatImpl: Sized + IDispatchImpl {
    fn Type();
    fn SetType();
    fn Guid();
    fn SetGuid();
    fn GetWaveFormatEx();
    fn SetWaveFormatEx();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechAudioStatusImpl: Sized + IDispatchImpl {
    fn FreeBufferSpace();
    fn NonBlockingIO();
    fn State();
    fn CurrentSeekPosition();
    fn CurrentDevicePosition();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechBaseStreamImpl: Sized + IDispatchImpl {
    fn Format();
    fn putref_Format();
    fn Read();
    fn Write();
    fn Seek();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechCustomStreamImpl: Sized + ISpeechBaseStreamImpl + IDispatchImpl {
    fn BaseStream();
    fn putref_BaseStream();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechDataKeyImpl: Sized + IDispatchImpl {
    fn SetBinaryValue();
    fn GetBinaryValue();
    fn SetStringValue();
    fn GetStringValue();
    fn SetLongValue();
    fn GetLongValue();
    fn OpenKey();
    fn CreateKey();
    fn DeleteKey();
    fn DeleteValue();
    fn EnumKeys();
    fn EnumValues();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechFileStreamImpl: Sized + ISpeechBaseStreamImpl + IDispatchImpl {
    fn Open();
    fn Close();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechGrammarRuleImpl: Sized + IDispatchImpl {
    fn Attributes();
    fn InitialState();
    fn Name();
    fn Id();
    fn Clear();
    fn AddResource();
    fn AddState();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechGrammarRuleStateImpl: Sized + IDispatchImpl {
    fn Rule();
    fn Transitions();
    fn AddWordTransition();
    fn AddRuleTransition();
    fn AddSpecialTransition();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechGrammarRuleStateTransitionImpl: Sized + IDispatchImpl {
    fn Type();
    fn Text();
    fn Rule();
    fn Weight();
    fn PropertyName();
    fn PropertyId();
    fn PropertyValue();
    fn NextState();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechGrammarRuleStateTransitionsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechGrammarRulesImpl: Sized + IDispatchImpl {
    fn Count();
    fn FindRule();
    fn Item();
    fn _NewEnum();
    fn Dynamic();
    fn Add();
    fn Commit();
    fn CommitAndSave();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechLexiconImpl: Sized + IDispatchImpl {
    fn GenerationId();
    fn GetWords();
    fn AddPronunciation();
    fn AddPronunciationByPhoneIds();
    fn RemovePronunciation();
    fn RemovePronunciationByPhoneIds();
    fn GetPronunciations();
    fn GetGenerationChange();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechLexiconPronunciationImpl: Sized + IDispatchImpl {
    fn Type();
    fn LangId();
    fn PartOfSpeech();
    fn PhoneIds();
    fn Symbolic();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechLexiconPronunciationsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechLexiconWordImpl: Sized + IDispatchImpl {
    fn LangId();
    fn Type();
    fn Word();
    fn Pronunciations();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechLexiconWordsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechMMSysAudioImpl: Sized + ISpeechAudioImpl + ISpeechBaseStreamImpl + IDispatchImpl {
    fn DeviceId();
    fn SetDeviceId();
    fn LineId();
    fn SetLineId();
    fn MMHandle();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechMemoryStreamImpl: Sized + ISpeechBaseStreamImpl + IDispatchImpl {
    fn SetData();
    fn GetData();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechObjectTokenImpl: Sized + IDispatchImpl {
    fn Id();
    fn DataKey();
    fn Category();
    fn GetDescription();
    fn SetId();
    fn GetAttribute();
    fn CreateInstance();
    fn Remove();
    fn GetStorageFileName();
    fn RemoveStorageFileName();
    fn IsUISupported();
    fn DisplayUI();
    fn MatchesAttributes();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechObjectTokenCategoryImpl: Sized + IDispatchImpl {
    fn Id();
    fn SetDefault();
    fn Default();
    fn SetId();
    fn GetDataKey();
    fn EnumerateTokens();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechObjectTokensImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhoneConverterImpl: Sized + IDispatchImpl {
    fn LanguageId();
    fn SetLanguageId();
    fn PhoneToId();
    fn IdToPhone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhraseAlternateImpl: Sized + IDispatchImpl {
    fn RecoResult();
    fn StartElementInResult();
    fn NumberOfElementsInResult();
    fn PhraseInfo();
    fn Commit();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhraseAlternatesImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhraseElementImpl: Sized + IDispatchImpl {
    fn AudioTimeOffset();
    fn AudioSizeTime();
    fn AudioStreamOffset();
    fn AudioSizeBytes();
    fn RetainedStreamOffset();
    fn RetainedSizeBytes();
    fn DisplayText();
    fn LexicalForm();
    fn Pronunciation();
    fn DisplayAttributes();
    fn RequiredConfidence();
    fn ActualConfidence();
    fn EngineConfidence();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhraseElementsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhraseInfoImpl: Sized + IDispatchImpl {
    fn LanguageId();
    fn GrammarId();
    fn StartTime();
    fn AudioStreamPosition();
    fn AudioSizeBytes();
    fn RetainedSizeBytes();
    fn AudioSizeTime();
    fn Rule();
    fn Properties();
    fn Elements();
    fn Replacements();
    fn EngineId();
    fn EnginePrivateData();
    fn SaveToMemory();
    fn GetText();
    fn GetDisplayAttributes();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhraseInfoBuilderImpl: Sized + IDispatchImpl {
    fn RestorePhraseFromMemory();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhrasePropertiesImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhrasePropertyImpl: Sized + IDispatchImpl {
    fn Name();
    fn Id();
    fn Value();
    fn FirstElement();
    fn NumberOfElements();
    fn EngineConfidence();
    fn Confidence();
    fn Parent();
    fn Children();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhraseReplacementImpl: Sized + IDispatchImpl {
    fn DisplayAttributes();
    fn Text();
    fn FirstElement();
    fn NumberOfElements();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhraseReplacementsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhraseRuleImpl: Sized + IDispatchImpl {
    fn Name();
    fn Id();
    fn FirstElement();
    fn NumberOfElements();
    fn Parent();
    fn Children();
    fn Confidence();
    fn EngineConfidence();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechPhraseRulesImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechRecoContextImpl: Sized + IDispatchImpl {
    fn Recognizer();
    fn AudioInputInterferenceStatus();
    fn RequestedUIType();
    fn putref_Voice();
    fn Voice();
    fn SetAllowVoiceFormatMatchingOnNextSet();
    fn AllowVoiceFormatMatchingOnNextSet();
    fn SetVoicePurgeEvent();
    fn VoicePurgeEvent();
    fn SetEventInterests();
    fn EventInterests();
    fn SetCmdMaxAlternates();
    fn CmdMaxAlternates();
    fn SetState();
    fn State();
    fn SetRetainedAudio();
    fn RetainedAudio();
    fn putref_RetainedAudioFormat();
    fn RetainedAudioFormat();
    fn Pause();
    fn Resume();
    fn CreateGrammar();
    fn CreateResultFromMemory();
    fn Bookmark();
    fn SetAdaptationData();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechRecoGrammarImpl: Sized + IDispatchImpl {
    fn Id();
    fn RecoContext();
    fn SetState();
    fn State();
    fn Rules();
    fn Reset();
    fn CmdLoadFromFile();
    fn CmdLoadFromObject();
    fn CmdLoadFromResource();
    fn CmdLoadFromMemory();
    fn CmdLoadFromProprietaryGrammar();
    fn CmdSetRuleState();
    fn CmdSetRuleIdState();
    fn DictationLoad();
    fn DictationUnload();
    fn DictationSetState();
    fn SetWordSequenceData();
    fn SetTextSelection();
    fn IsPronounceable();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechRecoResultImpl: Sized + IDispatchImpl {
    fn RecoContext();
    fn Times();
    fn putref_AudioFormat();
    fn AudioFormat();
    fn PhraseInfo();
    fn Alternates();
    fn Audio();
    fn SpeakAudio();
    fn SaveToMemory();
    fn DiscardResultInfo();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechRecoResult2Impl: Sized + ISpeechRecoResultImpl + IDispatchImpl {
    fn SetTextFeedback();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechRecoResultDispatchImpl: Sized + IDispatchImpl {
    fn RecoContext();
    fn Times();
    fn putref_AudioFormat();
    fn AudioFormat();
    fn PhraseInfo();
    fn Alternates();
    fn Audio();
    fn SpeakAudio();
    fn SaveToMemory();
    fn DiscardResultInfo();
    fn GetXMLResult();
    fn GetXMLErrorInfo();
    fn SetTextFeedback();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechRecoResultTimesImpl: Sized + IDispatchImpl {
    fn StreamTime();
    fn Length();
    fn TickCount();
    fn OffsetFromStart();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechRecognizerImpl: Sized + IDispatchImpl {
    fn putref_Recognizer();
    fn Recognizer();
    fn SetAllowAudioInputFormatChangesOnNextSet();
    fn AllowAudioInputFormatChangesOnNextSet();
    fn putref_AudioInput();
    fn AudioInput();
    fn putref_AudioInputStream();
    fn AudioInputStream();
    fn IsShared();
    fn SetState();
    fn State();
    fn Status();
    fn putref_Profile();
    fn Profile();
    fn EmulateRecognition();
    fn CreateRecoContext();
    fn GetFormat();
    fn SetPropertyNumber();
    fn GetPropertyNumber();
    fn SetPropertyString();
    fn GetPropertyString();
    fn IsUISupported();
    fn DisplayUI();
    fn GetRecognizers();
    fn GetAudioInputs();
    fn GetProfiles();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechRecognizerStatusImpl: Sized + IDispatchImpl {
    fn AudioStatus();
    fn CurrentStreamPosition();
    fn CurrentStreamNumber();
    fn NumberOfActiveRules();
    fn ClsidEngine();
    fn SupportedLanguages();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechResourceLoaderImpl: Sized + IDispatchImpl {
    fn LoadResource();
    fn GetLocalCopy();
    fn ReleaseLocalCopy();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechTextSelectionInformationImpl: Sized + IDispatchImpl {
    fn SetActiveOffset();
    fn ActiveOffset();
    fn SetActiveLength();
    fn ActiveLength();
    fn SetSelectionOffset();
    fn SelectionOffset();
    fn SetSelectionLength();
    fn SelectionLength();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechVoiceImpl: Sized + IDispatchImpl {
    fn Status();
    fn Voice();
    fn putref_Voice();
    fn AudioOutput();
    fn putref_AudioOutput();
    fn AudioOutputStream();
    fn putref_AudioOutputStream();
    fn Rate();
    fn SetRate();
    fn Volume();
    fn SetVolume();
    fn SetAllowAudioOutputFormatChangesOnNextSet();
    fn AllowAudioOutputFormatChangesOnNextSet();
    fn EventInterests();
    fn SetEventInterests();
    fn SetPriority();
    fn Priority();
    fn SetAlertBoundary();
    fn AlertBoundary();
    fn SetSynchronousSpeakTimeout();
    fn SynchronousSpeakTimeout();
    fn Speak();
    fn SpeakStream();
    fn Pause();
    fn Resume();
    fn Skip();
    fn GetVoices();
    fn GetAudioOutputs();
    fn WaitUntilDone();
    fn SpeakCompleteEvent();
    fn IsUISupported();
    fn DisplayUI();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechVoiceStatusImpl: Sized + IDispatchImpl {
    fn CurrentStreamNumber();
    fn LastStreamNumberQueued();
    fn LastHResult();
    fn RunningState();
    fn InputWordPosition();
    fn InputWordLength();
    fn InputSentencePosition();
    fn InputSentenceLength();
    fn LastBookmark();
    fn LastBookmarkId();
    fn PhonemeId();
    fn VisemeId();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechWaveFormatExImpl: Sized + IDispatchImpl {
    fn FormatTag();
    fn SetFormatTag();
    fn Channels();
    fn SetChannels();
    fn SamplesPerSec();
    fn SetSamplesPerSec();
    fn AvgBytesPerSec();
    fn SetAvgBytesPerSec();
    fn BlockAlign();
    fn SetBlockAlign();
    fn BitsPerSample();
    fn SetBitsPerSample();
    fn ExtraData();
    fn SetExtraData();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpeechXMLRecoResultImpl: Sized + ISpeechRecoResultImpl + IDispatchImpl {
    fn GetXMLResult();
    fn GetXMLErrorInfo();
}
#[cfg(feature = "Win32_System_Com")]
pub trait _ISpeechRecoContextEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait _ISpeechVoiceEventsImpl: Sized + IDispatchImpl {}
