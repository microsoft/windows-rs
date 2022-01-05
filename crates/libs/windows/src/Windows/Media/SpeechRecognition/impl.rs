#[cfg(feature = "implement_exclusive")]
pub trait ISpeechContinuousRecognitionCompletedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SpeechRecognitionResultStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechContinuousRecognitionResultGeneratedEventArgsImpl: Sized {
    fn Result(&self) -> ::windows::core::Result<SpeechRecognitionResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechContinuousRecognitionSessionImpl: Sized {
    fn AutoStopSilenceTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetAutoStopSilenceTimeout(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartWithModeAsync(&self, mode: SpeechContinuousRecognitionMode) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CancelAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn PauseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn Completed(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpeechContinuousRecognitionSession, SpeechContinuousRecognitionCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, value: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ResultGenerated(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpeechContinuousRecognitionSession, SpeechContinuousRecognitionResultGeneratedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveResultGenerated(&self, value: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionCompilationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SpeechRecognitionResultStatus>;
}
pub trait ISpeechRecognitionConstraintImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Type(&self) -> ::windows::core::Result<SpeechRecognitionConstraintType>;
    fn Probability(&self) -> ::windows::core::Result<SpeechRecognitionConstraintProbability>;
    fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionGrammarFileConstraintImpl: Sized + ISpeechRecognitionConstraintImpl {
    fn GrammarFile(&self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionGrammarFileConstraintFactoryImpl: Sized {
    fn Create(&self, file: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<SpeechRecognitionGrammarFileConstraint>;
    fn CreateWithTag(&self, file: &::core::option::Option<super::super::Storage::StorageFile>, tag: &::windows::core::HSTRING) -> ::windows::core::Result<SpeechRecognitionGrammarFileConstraint>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionHypothesisImpl: Sized {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionHypothesisGeneratedEventArgsImpl: Sized {
    fn Hypothesis(&self) -> ::windows::core::Result<SpeechRecognitionHypothesis>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionListConstraintImpl: Sized + ISpeechRecognitionConstraintImpl {
    fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionListConstraintFactoryImpl: Sized {
    fn Create(&self, commands: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<SpeechRecognitionListConstraint>;
    fn CreateWithTag(&self, commands: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, tag: &::windows::core::HSTRING) -> ::windows::core::Result<SpeechRecognitionListConstraint>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionQualityDegradingEventArgsImpl: Sized {
    fn Problem(&self) -> ::windows::core::Result<SpeechRecognitionAudioProblem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SpeechRecognitionResultStatus>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Confidence(&self) -> ::windows::core::Result<SpeechRecognitionConfidence>;
    fn SemanticInterpretation(&self) -> ::windows::core::Result<SpeechRecognitionSemanticInterpretation>;
    fn GetAlternates(&self, maxalternates: u32) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SpeechRecognitionResult>>;
    fn Constraint(&self) -> ::windows::core::Result<ISpeechRecognitionConstraint>;
    fn RulePath(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn RawConfidence(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionResult2Impl: Sized {
    fn PhraseStartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn PhraseDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionSemanticInterpretationImpl: Sized {
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionTopicConstraintImpl: Sized + ISpeechRecognitionConstraintImpl {
    fn Scenario(&self) -> ::windows::core::Result<SpeechRecognitionScenario>;
    fn TopicHint(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionTopicConstraintFactoryImpl: Sized {
    fn Create(&self, scenario: SpeechRecognitionScenario, topichint: &::windows::core::HSTRING) -> ::windows::core::Result<SpeechRecognitionTopicConstraint>;
    fn CreateWithTag(&self, scenario: SpeechRecognitionScenario, topichint: &::windows::core::HSTRING, tag: &::windows::core::HSTRING) -> ::windows::core::Result<SpeechRecognitionTopicConstraint>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionVoiceCommandDefinitionConstraintImpl: Sized + ISpeechRecognitionConstraintImpl {}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISpeechRecognizerImpl: Sized + IClosableImpl {
    fn CurrentLanguage(&self) -> ::windows::core::Result<super::super::Globalization::Language>;
    fn Constraints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ISpeechRecognitionConstraint>>;
    fn Timeouts(&self) -> ::windows::core::Result<SpeechRecognizerTimeouts>;
    fn UIOptions(&self) -> ::windows::core::Result<SpeechRecognizerUIOptions>;
    fn CompileConstraintsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechRecognitionCompilationResult>>;
    fn RecognizeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechRecognitionResult>>;
    fn RecognizeWithUIAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechRecognitionResult>>;
    fn RecognitionQualityDegrading(&self, speechrecognitionqualitydegradinghandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognitionQualityDegradingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRecognitionQualityDegrading(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StateChanged(&self, statechangedhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognizerStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognizer2Impl: Sized {
    fn ContinuousRecognitionSession(&self) -> ::windows::core::Result<SpeechContinuousRecognitionSession>;
    fn State(&self) -> ::windows::core::Result<SpeechRecognizerState>;
    fn StopRecognitionAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn HypothesisGenerated(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognitionHypothesisGeneratedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHypothesisGenerated(&self, value: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognizerFactoryImpl: Sized {
    fn Create(&self, language: &::core::option::Option<super::super::Globalization::Language>) -> ::windows::core::Result<SpeechRecognizer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognizerStateChangedEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<SpeechRecognizerState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognizerStaticsImpl: Sized {
    fn SystemSpeechLanguage(&self) -> ::windows::core::Result<super::super::Globalization::Language>;
    fn SupportedTopicLanguages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>>;
    fn SupportedGrammarLanguages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognizerStatics2Impl: Sized {
    fn TrySetSystemSpeechLanguageAsync(&self, speechlanguage: &::core::option::Option<super::super::Globalization::Language>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognizerTimeoutsImpl: Sized {
    fn InitialSilenceTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetInitialSilenceTimeout(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn EndSilenceTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetEndSilenceTimeout(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn BabbleTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetBabbleTimeout(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognizerUIOptionsImpl: Sized {
    fn ExampleText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetExampleText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AudiblePrompt(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAudiblePrompt(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsReadBackEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsReadBackEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ShowConfirmation(&self) -> ::windows::core::Result<bool>;
    fn SetShowConfirmation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandManagerImpl: Sized {
    fn InstallCommandSetsFromStorageFileAsync(&self, file: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn InstalledCommandSets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, VoiceCommandSet>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandSetImpl: Sized {
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPhraseListAsync(&self, phraselistname: &::windows::core::HSTRING, phraselist: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
