#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISpeechContinuousRecognitionCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechContinuousRecognitionCompletedEventArgs {}
impl ::core::clone::Clone for ISpeechContinuousRecognitionCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechContinuousRecognitionResultGeneratedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechContinuousRecognitionResultGeneratedEventArgs {}
impl ::core::clone::Clone for ISpeechContinuousRecognitionResultGeneratedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechContinuousRecognitionSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechContinuousRecognitionSession {}
impl ::core::clone::Clone for ISpeechContinuousRecognitionSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognitionCompilationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognitionCompilationResult {}
impl ::core::clone::Clone for ISpeechRecognitionCompilationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognitionConstraint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognitionConstraint {}
impl ::core::clone::Clone for ISpeechRecognitionConstraint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognitionGrammarFileConstraint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognitionGrammarFileConstraint {}
impl ::core::clone::Clone for ISpeechRecognitionGrammarFileConstraint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognitionGrammarFileConstraintFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognitionGrammarFileConstraintFactory {}
impl ::core::clone::Clone for ISpeechRecognitionGrammarFileConstraintFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognitionHypothesis(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognitionHypothesis {}
impl ::core::clone::Clone for ISpeechRecognitionHypothesis {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognitionHypothesisGeneratedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognitionHypothesisGeneratedEventArgs {}
impl ::core::clone::Clone for ISpeechRecognitionHypothesisGeneratedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognitionListConstraint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognitionListConstraint {}
impl ::core::clone::Clone for ISpeechRecognitionListConstraint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognitionListConstraintFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognitionListConstraintFactory {}
impl ::core::clone::Clone for ISpeechRecognitionListConstraintFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognitionQualityDegradingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognitionQualityDegradingEventArgs {}
impl ::core::clone::Clone for ISpeechRecognitionQualityDegradingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognitionResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognitionResult {}
impl ::core::clone::Clone for ISpeechRecognitionResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognitionResult2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognitionResult2 {}
impl ::core::clone::Clone for ISpeechRecognitionResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognitionSemanticInterpretation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognitionSemanticInterpretation {}
impl ::core::clone::Clone for ISpeechRecognitionSemanticInterpretation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognitionTopicConstraint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognitionTopicConstraint {}
impl ::core::clone::Clone for ISpeechRecognitionTopicConstraint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognitionTopicConstraintFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognitionTopicConstraintFactory {}
impl ::core::clone::Clone for ISpeechRecognitionTopicConstraintFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognitionVoiceCommandDefinitionConstraint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognitionVoiceCommandDefinitionConstraint {}
impl ::core::clone::Clone for ISpeechRecognitionVoiceCommandDefinitionConstraint {
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
pub struct ISpeechRecognizer2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognizer2 {}
impl ::core::clone::Clone for ISpeechRecognizer2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognizerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognizerFactory {}
impl ::core::clone::Clone for ISpeechRecognizerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognizerStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognizerStateChangedEventArgs {}
impl ::core::clone::Clone for ISpeechRecognizerStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognizerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognizerStatics {}
impl ::core::clone::Clone for ISpeechRecognizerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognizerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognizerStatics2 {}
impl ::core::clone::Clone for ISpeechRecognizerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognizerTimeouts(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognizerTimeouts {}
impl ::core::clone::Clone for ISpeechRecognizerTimeouts {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpeechRecognizerUIOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpeechRecognizerUIOptions {}
impl ::core::clone::Clone for ISpeechRecognizerUIOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoiceCommandManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoiceCommandManager {}
impl ::core::clone::Clone for IVoiceCommandManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVoiceCommandSet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVoiceCommandSet {}
impl ::core::clone::Clone for IVoiceCommandSet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechContinuousRecognitionCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpeechContinuousRecognitionCompletedEventArgs {}
impl ::core::clone::Clone for SpeechContinuousRecognitionCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechContinuousRecognitionMode(pub i32);
impl SpeechContinuousRecognitionMode {
    pub const Default: Self = Self(0i32);
    pub const PauseOnRecognition: Self = Self(1i32);
}
impl ::core::marker::Copy for SpeechContinuousRecognitionMode {}
impl ::core::clone::Clone for SpeechContinuousRecognitionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechContinuousRecognitionResultGeneratedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpeechContinuousRecognitionResultGeneratedEventArgs {}
impl ::core::clone::Clone for SpeechContinuousRecognitionResultGeneratedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechContinuousRecognitionSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpeechContinuousRecognitionSession {}
impl ::core::clone::Clone for SpeechContinuousRecognitionSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognitionAudioProblem(pub i32);
impl SpeechRecognitionAudioProblem {
    pub const None: Self = Self(0i32);
    pub const TooNoisy: Self = Self(1i32);
    pub const NoSignal: Self = Self(2i32);
    pub const TooLoud: Self = Self(3i32);
    pub const TooQuiet: Self = Self(4i32);
    pub const TooFast: Self = Self(5i32);
    pub const TooSlow: Self = Self(6i32);
}
impl ::core::marker::Copy for SpeechRecognitionAudioProblem {}
impl ::core::clone::Clone for SpeechRecognitionAudioProblem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognitionCompilationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpeechRecognitionCompilationResult {}
impl ::core::clone::Clone for SpeechRecognitionCompilationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognitionConfidence(pub i32);
impl SpeechRecognitionConfidence {
    pub const High: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Low: Self = Self(2i32);
    pub const Rejected: Self = Self(3i32);
}
impl ::core::marker::Copy for SpeechRecognitionConfidence {}
impl ::core::clone::Clone for SpeechRecognitionConfidence {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognitionConstraintProbability(pub i32);
impl SpeechRecognitionConstraintProbability {
    pub const Default: Self = Self(0i32);
    pub const Min: Self = Self(1i32);
    pub const Max: Self = Self(2i32);
}
impl ::core::marker::Copy for SpeechRecognitionConstraintProbability {}
impl ::core::clone::Clone for SpeechRecognitionConstraintProbability {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognitionConstraintType(pub i32);
impl SpeechRecognitionConstraintType {
    pub const Topic: Self = Self(0i32);
    pub const List: Self = Self(1i32);
    pub const Grammar: Self = Self(2i32);
    pub const VoiceCommandDefinition: Self = Self(3i32);
}
impl ::core::marker::Copy for SpeechRecognitionConstraintType {}
impl ::core::clone::Clone for SpeechRecognitionConstraintType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognitionGrammarFileConstraint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpeechRecognitionGrammarFileConstraint {}
impl ::core::clone::Clone for SpeechRecognitionGrammarFileConstraint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognitionHypothesis(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpeechRecognitionHypothesis {}
impl ::core::clone::Clone for SpeechRecognitionHypothesis {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognitionHypothesisGeneratedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpeechRecognitionHypothesisGeneratedEventArgs {}
impl ::core::clone::Clone for SpeechRecognitionHypothesisGeneratedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognitionListConstraint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpeechRecognitionListConstraint {}
impl ::core::clone::Clone for SpeechRecognitionListConstraint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognitionQualityDegradingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpeechRecognitionQualityDegradingEventArgs {}
impl ::core::clone::Clone for SpeechRecognitionQualityDegradingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognitionResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpeechRecognitionResult {}
impl ::core::clone::Clone for SpeechRecognitionResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognitionResultStatus(pub i32);
impl SpeechRecognitionResultStatus {
    pub const Success: Self = Self(0i32);
    pub const TopicLanguageNotSupported: Self = Self(1i32);
    pub const GrammarLanguageMismatch: Self = Self(2i32);
    pub const GrammarCompilationFailure: Self = Self(3i32);
    pub const AudioQualityFailure: Self = Self(4i32);
    pub const UserCanceled: Self = Self(5i32);
    pub const Unknown: Self = Self(6i32);
    pub const TimeoutExceeded: Self = Self(7i32);
    pub const PauseLimitExceeded: Self = Self(8i32);
    pub const NetworkFailure: Self = Self(9i32);
    pub const MicrophoneUnavailable: Self = Self(10i32);
}
impl ::core::marker::Copy for SpeechRecognitionResultStatus {}
impl ::core::clone::Clone for SpeechRecognitionResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognitionScenario(pub i32);
impl SpeechRecognitionScenario {
    pub const WebSearch: Self = Self(0i32);
    pub const Dictation: Self = Self(1i32);
    pub const FormFilling: Self = Self(2i32);
}
impl ::core::marker::Copy for SpeechRecognitionScenario {}
impl ::core::clone::Clone for SpeechRecognitionScenario {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognitionSemanticInterpretation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpeechRecognitionSemanticInterpretation {}
impl ::core::clone::Clone for SpeechRecognitionSemanticInterpretation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognitionTopicConstraint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpeechRecognitionTopicConstraint {}
impl ::core::clone::Clone for SpeechRecognitionTopicConstraint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognitionVoiceCommandDefinitionConstraint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpeechRecognitionVoiceCommandDefinitionConstraint {}
impl ::core::clone::Clone for SpeechRecognitionVoiceCommandDefinitionConstraint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognizer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpeechRecognizer {}
impl ::core::clone::Clone for SpeechRecognizer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognizerState(pub i32);
impl SpeechRecognizerState {
    pub const Idle: Self = Self(0i32);
    pub const Capturing: Self = Self(1i32);
    pub const Processing: Self = Self(2i32);
    pub const SoundStarted: Self = Self(3i32);
    pub const SoundEnded: Self = Self(4i32);
    pub const SpeechDetected: Self = Self(5i32);
    pub const Paused: Self = Self(6i32);
}
impl ::core::marker::Copy for SpeechRecognizerState {}
impl ::core::clone::Clone for SpeechRecognizerState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognizerStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpeechRecognizerStateChangedEventArgs {}
impl ::core::clone::Clone for SpeechRecognizerStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognizerTimeouts(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpeechRecognizerTimeouts {}
impl ::core::clone::Clone for SpeechRecognizerTimeouts {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpeechRecognizerUIOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpeechRecognizerUIOptions {}
impl ::core::clone::Clone for SpeechRecognizerUIOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VoiceCommandSet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VoiceCommandSet {}
impl ::core::clone::Clone for VoiceCommandSet {
    fn clone(&self) -> Self {
        *self
    }
}
